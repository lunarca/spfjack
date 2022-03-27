mod spf;
mod dns;

use clap::{Arg, App, ArgMatches};


#[tokio::main]
async fn main() {

    let args_matcher = parse_args();

    let domain = String::from(args_matcher.value_of("domain").unwrap());

    let resolver = dns::dns_resolver::new_resolver();

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