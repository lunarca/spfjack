use std::sync::Arc;

use decon_spf::{mechanism::{Mechanism, Kind}, Spf};

use futures::future::join_all;
use trust_dns_resolver::TokioAsyncResolver;

use crate::dns::dns_resolver::is_domain_registered;

#[derive(Debug)]
pub struct MechanismProcessingResult {
    mechanism_type: Kind,
    issue: MisconfigType,
    mechanism: String
}


#[derive(Debug)]
pub enum MisconfigType {
    /// Mechanism is +all
    PlusAll,
    /// Mechanism points to an open relay
    OpenRelay(String), //Consider union type of DNS name or IP4/6 address
    /// Mechanism points to an unregistered domain
    UnregisteredDomain(String),
    /// No SPF record
    NoSpfRecord
}

pub async fn process_spf_record(resolver: &TokioAsyncResolver, spf: Arc<Spf>) -> Vec<MechanismProcessingResult> {

    let include_mechanisms = spf.includes();
    let include_mechanism_results = match include_mechanisms {
        Some(include_mechanisms) => process_include_mechanisms(resolver, include_mechanisms).await,
        None => vec![]
    };

    let all_mechanism_results = spf.all().and_then(process_all_mechanism);

    include_mechanism_results
}

async fn process_include_mechanisms(resolver: &TokioAsyncResolver, mechanisms: &Vec<Mechanism<String>>) -> Vec<MechanismProcessingResult> {
    join_all(mechanisms
        .iter()
        .map(|mechanism| process_include_mechanism(resolver, mechanism))
    ).await
    .into_iter()
    .flatten()
    .collect()
}

pub async fn process_include_mechanism(resolver: &TokioAsyncResolver, mechanism: &Mechanism<String>) -> Vec<MechanismProcessingResult> {
    info!("Processing include mechanism: `{}`", mechanism.to_string());

    match mechanism.mechanism() {
        Some(domain) => {
            if is_domain_registered(resolver, domain).await {
                info!("Include domain `{}` registered. Recursively processing. TODO: Not currently done", domain);
                // TODO: handle recursive case
                vec![]
            } else {
                info!("Include domain `{}` not registered. Returning as a processing result.", domain);
                return vec![MechanismProcessingResult{
                    mechanism_type: Kind::Include,
                    issue: MisconfigType::UnregisteredDomain(domain.to_owned()),
                    mechanism: mechanism.to_string()
                }]
            }
            }
        None => {
            warn!("Include mechanism failed to have a proper resolution: {}", mechanism.to_string());
            vec![]
        }
    }

    
}

pub fn process_all_mechanism(mechanism: &Mechanism<String>) -> Option<MechanismProcessingResult> {
    trace!("Calling process_all_mechanism with {}", mechanism.to_string());
    if mechanism.is_pass() && matches!(mechanism.kind(), Kind::All) {
        info!("Found a +all mechanism");
        return Some(MechanismProcessingResult{
            mechanism_type: Kind::All,
            mechanism: mechanism.to_string(),
            issue: MisconfigType::PlusAll
        });
    } else {
        info!("No +all mechanism");
        None
    }
}