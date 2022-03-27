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
        Some(record) => Ok(Arc::clone(record)),

        // Cache miss: Fetch the SPF record, then add to cache
        None => {
            let optional_spf_record = spf::fetch_and_parse(resolver, domain.clone())
                .await
                .map(|record| Arc::new(record))
                .and_then(|record| {
                    cache.insert(domain.clone(), Arc::clone(&record));
                    Ok(record)
                });
            
            optional_spf_record
        }
    }
}

pub async fn is_domain_registered(resolver: &TokioAsyncResolver, domain: &String) -> bool {
    match resolver.ns_lookup(domain).await {
        Ok(_) => true,
        Err(err) => {
            match err.kind() {
                ResolveErrorKind::NoRecordsFound {  response_code, ..} if *response_code == ResponseCode::NXDomain => {
                    false
                }
                _ => true
            }
        }
    }
}