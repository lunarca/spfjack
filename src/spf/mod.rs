
use decon_spf::spf::Spf;
use trust_dns_resolver::error::ResolveResult;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::{config::*, lookup::TxtLookup};

pub fn fetch_and_parse(domain: &str) -> Result<Spf, String> {
  fetch_txt_records(domain)
    .and_then(select_spf_record)
}

fn fetch_txt_records(domain: &str) -> Result<TxtLookup, String> {
  let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
  match resolver.txt_lookup(domain) {
    Err(_) => Err(String::from("No TXT records")),
    Ok(txt_record) => Ok(txt_record)
  }
}

fn select_spf_record(txt_records: TxtLookup) -> Result<Spf, String> {
  for record in txt_records.iter() {
    if record.to_string().starts_with("v=spf1") {
      let mut record = Spf::from_str(&record.to_string());
      return match record.parse() {
        Ok(_) => Ok(record),
        Err(_) => Err(String::from("Error decoding SPF record"))
      }
    }
  }
  return Err(String::from("No SPF record"))
}