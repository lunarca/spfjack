use actix::prelude::*;
use trust_dns_resolver::Resolver;


//------
// Actor definition
pub struct DnsResolverActor {
    resolver: Resolver,
}

impl Actor for DnsResolverActor {
    type Context = Context<Self>;
}

//-----
// Actor message handlers


//-----
// Message Definitions


