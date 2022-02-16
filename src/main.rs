mod spf;
mod actors;

use actix::System;
use clap::{Arg, App, ArgMatches};
use trust_dns_resolver::Resolver;
use trust_dns_resolver::{config::*};
use actix_rt;

use actors::spf_cache;


#[actix_rt::main]
async fn main() {
    let args_matcher = parse_args();

    let spf_cache_addr = spf_cache::start_link();

    let domain: String = String::from(args_matcher.value_of("domain").unwrap());

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let spf_record = spf::fetch_and_parse(resolver, domain);
    println!("SPF Record: {:?}", spf_record);

    System::current().stop();
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