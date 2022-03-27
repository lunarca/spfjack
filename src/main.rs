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

pub type SpfCache = Arc<Mutex<HashMap<String, Arc<Spf>>>>;

#[tokio::main]
async fn main() {

    let args_matcher = parse_args();

    let domain = String::from(args_matcher.value_of("domain").unwrap());

    println!("Targeting domain: {}", domain);

    let cache: SpfCache = Arc::new(Mutex::new(HashMap::new()));

    let resolver = new_resolver();

    let record = resolve_spf_record(&domain, &resolver, cache).await.unwrap();

    println!("Fetched record: {}", record.to_string());

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