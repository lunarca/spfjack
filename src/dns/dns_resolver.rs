use std::sync::Arc;

use decon_spf::Spf;
use trust_dns_resolver::{
    TokioAsyncResolver,
    TokioHandle, 
    error::{
        ResolveErrorKind
    }, proto::op::ResponseCode,
};

use crate::{SpfCache, spf::{self, SpfFetchError}};



pub fn new_resolver() -> TokioAsyncResolver {
    TokioAsyncResolver::from_system_conf(TokioHandle).unwrap()
}

pub async fn resolve_spf_record(domain: &String, resolver: &TokioAsyncResolver, cache: SpfCache) -> Result<Arc<Spf>, SpfFetchError> {
    let mut cache = cache.lock().unwrap();

    match cache.get(domain) {
        // Cache hit: Just return the resulting SPF record
        Some(record) => {
            debug!("Cache hit: {}", domain);
            Ok(Arc::clone(record))
        },

        // Cache miss: Fetch the SPF record, then add to cache
        None => {
            debug!("Cache miss: {}", domain);
            let optional_spf_record = spf::fetch_and_parse(resolver, domain.clone())
                .await
                .map(|record| Arc::new(record))
                .and_then(|record| {
                    debug!("Got record for domain {}, adding to cache: {}", domain, record.to_string());
                    cache.insert(domain.clone(), Arc::clone(&record));
                    Ok(record)
                });
            
            optional_spf_record
        }
    }
}

pub async fn is_domain_registered(resolver: &TokioAsyncResolver, domain: &String) -> bool {
    trace!("is_domain_registered on {}", domain);
    match resolver.ns_lookup(domain).await {
        Ok(_) => true,
        Err(err) => {
            debug!("Resolve error: {:?}", err);
            match err.kind() {
                ResolveErrorKind::NoRecordsFound {  response_code, ..} if *response_code == ResponseCode::NXDomain => {
                    false
                }
                _ => true
            }
        }
    }
}