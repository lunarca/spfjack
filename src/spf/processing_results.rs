use std::{sync::Arc, rc::Rc};

use decon_spf::{mechanism::{Mechanism, Kind}, Spf};

use futures::future::join_all;
use trust_dns_resolver::TokioAsyncResolver;

use crate::dns::dns_resolver::is_domain_registered;

use super::SpfFetchError;

#[derive(Debug, Clone)]
pub struct MechanismProcessingResult {
    pub mechanism_type: Option<Kind>,
    pub issue: MisconfigType,
    pub mechanism: String,
    pub domain: String,
}


#[derive(Debug, Clone)]
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

pub async fn process_spf_record_result(resolver: TokioAsyncResolver, record_result: Result<Arc<Spf>, SpfFetchError>, domain: String) -> Vec<MechanismProcessingResult> {
    match record_result {
        Err(err) => {
            return vec!(MechanismProcessingResult {
                domain: domain.clone(),
                issue: MisconfigType::NoSpfRecord,
                mechanism: "".to_string(),
                mechanism_type: None
                
            })
        },
        Ok(record) => {
            info!("Got SPF Record: {}", record.to_string());
            info!("Starting to process data.");
            let processing_results = process_spf_record(&resolver, domain.clone(), record).await;
            info!("Results: {:#?}", processing_results);
            return processing_results
        }

    }
}

pub async fn process_spf_record(resolver: &TokioAsyncResolver, domain: String, spf: Arc<Spf>) -> Vec<MechanismProcessingResult> {

    let include_mechanisms = spf.includes();
    let include_mechanism_results = match include_mechanisms {
        Some(include_mechanisms) => Some(process_include_mechanisms(resolver, domain.clone(), include_mechanisms).await),
        None => None
    };

    let all_mechanism_domain = domain.clone();
    let all_mechanism_results = spf.all().and_then(move |mechanism| process_all_mechanism(all_mechanism_domain, mechanism));

    let all_results = vec!(
        include_mechanism_results,
        all_mechanism_results,
    );

    let filtered_results = all_results
        .iter()
        .flatten()
        .flatten()
        .map(|element| element.clone())
        .collect();

    return filtered_results
    
}

async fn process_include_mechanisms(resolver: &TokioAsyncResolver, domain: String, mechanisms: &Vec<Mechanism<String>>) -> Vec<MechanismProcessingResult> {
    join_all(mechanisms
        .iter()
        .map(|mechanism| process_include_mechanism(resolver, domain.clone(),  mechanism))
    ).await
    .into_iter()
    .flatten()
    .collect()
}

pub async fn process_include_mechanism(resolver: &TokioAsyncResolver, domain: String, mechanism: &Mechanism<String>) -> Vec<MechanismProcessingResult> {
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
                    mechanism_type: Some(Kind::Include),
                    issue: MisconfigType::UnregisteredDomain(domain.to_owned()),
                    mechanism: mechanism.to_string(),
                    domain: domain.clone()
                }]
            }
            }
        None => {
            warn!("Include mechanism failed to have a proper resolution: {}", mechanism.to_string());
            vec![]
        }
    }

    
}

pub fn process_all_mechanism(domain: String, mechanism: &Mechanism<String>) -> Option<Vec<MechanismProcessingResult>> {
    trace!("Calling process_all_mechanism with {}", mechanism.to_string());
    if mechanism.is_pass() && matches!(mechanism.kind(), Kind::All) {
        info!("Found a +all mechanism");
        return Some(vec!(MechanismProcessingResult{
            mechanism_type: Some(Kind::All),
            mechanism: mechanism.to_string(),
            issue: MisconfigType::PlusAll,
            domain: domain.clone()
        }));
    } else {
        info!("No +all mechanism");
        None
    }
}