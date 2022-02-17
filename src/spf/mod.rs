
use decon_spf::spf::{Spf, SpfErrorType};
use trust_dns_resolver::Resolver;
use trust_dns_resolver::lookup::TxtLookup;

pub mod processing_results;

pub fn fetch_and_parse(resolver: Resolver, domain: String) -> Result<Spf, SpfFetchError> {
  fetch_txt_records(resolver, domain)
    .and_then(select_spf_record)
}

fn fetch_txt_records(resolver: Resolver, domain: String) -> Result<TxtLookup, SpfFetchError> {
  match resolver.txt_lookup(domain) {
    Err(_) => Err(SpfFetchError::NoTxtRecords),
    Ok(txt_record) => Ok(txt_record)
  }
}

fn select_spf_record(txt_records: TxtLookup) -> Result<Spf, SpfFetchError> {
  for record in txt_records.iter() {
    if record.to_string().starts_with("v=spf1") {
      let mut record = Spf::from_str(&record.to_string());
      return match record.parse() {
        Ok(_) => Ok(record),
        Err(error) => Err(SpfFetchError::SpfParseError(error))
      }
    }
  }
  return Err(SpfFetchError::NoSpfRecords)
}

pub enum SpfFetchError {
  NoTxtRecords,
  NoSpfRecords,
  SpfParseError(SpfErrorType)
}