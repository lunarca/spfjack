mod spf;
mod actors;

use actix::System;
use clap::{Arg, App, ArgMatches};

use actors::spf_cache;
use actors::resolver;


#[actix::main]
async fn main() {
    let args_matcher = parse_args();

    let spf_cache_addr = spf_cache::start_link();

    let resolver_addr = resolver::start_link(&spf_cache_addr);

    let domain: String = String::from(args_matcher.value_of("domain").unwrap());


    let spf_record = resolver_addr.send(resolver::FetchSfpRecordMessage{dns_name: domain}).await.unwrap();
    match spf_record {
        Ok(record) => println!("SPF Record: {}", record.source()),
        Err(_error) => println!("Error")
    }

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