mod spf;
mod dns;

use clap::{Arg, App, ArgMatches};
use decon_spf::Spf;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use dns::dns_resolver::{
    new_resolver,
    resolve_spf_record,
};

use crate::spf::processing_results::{ process_spf_record_result};

extern crate pretty_env_logger;
#[macro_use] extern crate log;

pub type SpfCache = Arc<Mutex<HashMap<String, Arc<Spf>>>>;

#[tokio::main]
async fn main() {

    // Initialize the logger
    pretty_env_logger::init();

    let args_matcher = parse_args();

    let domain = String::from(args_matcher.value_of("domain").unwrap());

    println!("Targeting domain: {}", domain);

    let cache: SpfCache = Arc::new(Mutex::new(HashMap::new()));

    let resolver = new_resolver();

    let record = resolve_spf_record(&domain, &resolver, cache).await;

    let results = process_spf_record_result(resolver, record, domain).await;    
    println!("Results: {:?}", results);

}


fn parse_args() -> ArgMatches {
    let matches = App::new("SPFJack")
        .version("0.0.1")
        .author("LunarCA")
        .about("Review a domain's SPF records for misconfigurations")
        .arg(Arg::new("domain")
                .help("Domain to query")
                .required(true))
        .get_matches();
    return matches
}