use actix::prelude::*;

use super::resolver::DnsResolverActor;

//------
// Actor definition
pub struct SpfMechanismProcessorActor {
    dns_resolver_addr: Addr<DnsResolverActor>
}

impl Actor for SpfMechanismProcessorActor {
    type Context = Context<Self>;
}

//-----
// Message Definitions and actor handlers



//-----
/// Primary function to start the SpfMechanismProcessorActor Actor
pub fn start_link(dns_resolver_addr: &Addr<DnsResolverActor>) -> Addr<SpfMechanismProcessorActor> {
    return SpfMechanismProcessorActor { dns_resolver_addr: dns_resolver_addr.clone()  }.start();
}