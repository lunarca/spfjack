use actix::prelude::*;
use trust_dns_resolver::{Resolver, lookup::*, error::ResolveError};


//------
// Actor definition
pub struct DnsResolverActor {
    resolver: Resolver,
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
    type Result = ResolveTxtMessageResponse;

    fn handle(&mut self, msg: ResolveTxtMessage, _ctx: &mut Context<Self>) -> Self::Result {
        return self.resolver.txt_lookup(msg.dns_name);
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
    type Result = ResolveAMessageResponse;

    fn handle(&mut self, msg: ResolveAMessage, _ctx: &mut Context<Self>) -> Self::Result {
        return self.resolver.ipv4_lookup(msg.dns_name);
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
    type Result = ResolveAaaaMessageResponse;

    fn handle(&mut self, msg: ResolveAaaaMessage, _ctx: &mut Context<Self>) -> Self::Result {
        return self.resolver.ipv6_lookup(msg.dns_name);
    }
}