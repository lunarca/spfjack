use std::sync::Arc;

use actix::prelude::*;
use decon_spf::Spf;
use trust_dns_resolver::{
    TokioAsyncResolver, 
    TokioHandle,
    lookup::*, 
    error::ResolveError, 
};

use crate::spf::{self, SpfFetchError};

use super::spf_cache::{SpfCacheActor, QueryCacheMessage, InsertCacheMessage};


//------
// Actor definition
pub struct DnsResolverActor {
    resolver: TokioAsyncResolver,
    spf_cache_addr: Addr<SpfCacheActor>
}

impl Actor for DnsResolverActor {
    type Context = Context<Self>;
}

//-----
// Message Definitions and actor handlers

//-----
// Resolve a TXT Record

type ResolveTxtMessageResponse = Result<TxtLookup, ResolveError>;

pub struct ResolveTxtMessage {
    pub dns_name: String,
}

impl Message for ResolveTxtMessage {
    type Result = ResolveTxtMessageResponse;
}

impl Handler<ResolveTxtMessage> for DnsResolverActor {
    type Result = ResponseActFuture<Self, ResolveTxtMessageResponse>;

    fn handle(&mut self, msg: ResolveTxtMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let resolver = self.resolver.clone();
        let future = resolver
            .txt_lookup(msg.dns_name)
            .into_actor(self);

        return Box::pin(future)
    }
}

//-----
// Resolve an A Record
type ResolveAMessageResponse = Result<Ipv4Lookup, ResolveError>;

pub struct ResolveAMessage {
    pub dns_name: String,
}

impl Message for ResolveAMessage {
    type Result = ResolveAMessageResponse;
}

impl Handler<ResolveAMessage> for DnsResolverActor {
    type Result = ResponseActFuture<Self, ResolveAMessageResponse>;

    fn handle(&mut self, msg: ResolveAMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let resolver = self.resolver.clone();
        let future = resolver
            .ipv4_lookup(msg.dns_name)
            .into_actor(self);

        return Box::pin(future);
    }
}

//-----
// Resolve an AAAA Record
type ResolveAaaaMessageResponse = Result<Ipv6Lookup, ResolveError>;

pub struct ResolveAaaaMessage {
    pub dns_name: String,
}

impl Message for ResolveAaaaMessage {
    type Result = ResolveAaaaMessageResponse;
}

impl Handler<ResolveAaaaMessage> for DnsResolverActor {
    type Result = ResponseActFuture<Self, ResolveAaaaMessageResponse>;

    fn handle(&mut self, msg: ResolveAaaaMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let resolver = self.resolver.clone();
        let future = resolver
            .ipv6_lookup(msg.dns_name)
            .into_actor(self);
        
            return Box::pin(future)
    }
}

//-----
// Resolve an MX record
type ResolveMxMessageResponse = Result<MxLookup, ResolveError>;

pub struct ResolveMxMessage {
    pub dns_name: String,
}

impl Message for ResolveMxMessage {
    type Result = ResolveMxMessageResponse;
}

impl Handler<ResolveMxMessage> for DnsResolverActor {
    type Result = ResponseActFuture<Self, ResolveMxMessageResponse>;

    fn handle(&mut self, msg: ResolveMxMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let resolver = self.resolver.clone();
        let future = resolver
            .mx_lookup(msg.dns_name)
            .into_actor(self);

        return Box::pin(future)
    }
}

//-----
// Extract an SPF record for a domain
type FetchSfpRecordMessageResponse = Result<Arc<Spf>, SpfFetchError>;

pub struct FetchSfpRecordMessage {
    pub dns_name: String,
}

impl Message for FetchSfpRecordMessage {
    type Result = FetchSfpRecordMessageResponse;
}

impl Handler<FetchSfpRecordMessage> for DnsResolverActor {
    type Result = ResponseActFuture<Self, FetchSfpRecordMessageResponse>;

    fn handle(&mut self, msg: FetchSfpRecordMessage, _ctx: &mut Context<Self>) -> Self::Result {

        let result = self.spf_cache_addr
            .send(QueryCacheMessage{domain: msg.dns_name.to_owned() })
            .into_actor(self)
            .map(move |res, act, _ctx| {
                match res.unwrap() {
                    Some(record) => Ok(record),
                    None => {
                        match spf::fetch_and_parse(&act.resolver, msg.dns_name.clone()) {
                            Ok(record) => {
                                let record = Arc::new(record);
                                act.spf_cache_addr.do_send(InsertCacheMessage{domain: msg.dns_name.to_owned(), value: Arc::clone(&record)});
                                Ok(record)
                            },
                            Err(error) => Err(error)
                        }
                    }
                }
            });

        return Box::pin(result)
    }
}

//-----
/// Primary function to start the DnsResolverActor Actor
pub fn start_link(spf_cache_addr: &Addr<SpfCacheActor> ) -> Addr<DnsResolverActor> {
    return DnsResolverActor { 
        resolver: TokioAsyncResolver::from_system_conf(TokioHandle).unwrap(),
        spf_cache_addr: spf_cache_addr.clone(),
    }.start();
}