mod spf;

use trust_dns_resolver::Resolver;
use trust_dns_resolver::{config::*};



fn main() {
    let domain = String::from("bishopfox.com.");

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();


    let spf_record = spf::fetch_and_parse(resolver, domain);
    println!("SPF Record: {:?}", spf_record);
}
