mod spf;

use clap::{Arg, App, ArgMatches};


#[tokio::main]
async fn main() {

    let args_matcher = parse_args();


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