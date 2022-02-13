mod spf;

use clap::{Arg, App, ArgMatches};
use trust_dns_resolver::Resolver;
use trust_dns_resolver::{config::*};



fn main() {
    let args_matcher = parse_args();

    let domain: String = String::from(args_matcher.value_of("domain").unwrap());

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();


    let spf_record = spf::fetch_and_parse(resolver, domain);
    println!("SPF Record: {:?}", spf_record);
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