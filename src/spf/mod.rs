use decon_spf::{Spf, SpfError};
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::lookup::TxtLookup;
use regex::Regex;

pub mod processing_results;

pub async fn fetch_and_parse(resolver: &TokioAsyncResolver, domain: String) -> Result<Spf, SpfFetchError> {
  fetch_txt_records(resolver, domain)
    .await
    .and_then(select_spf_record)
}

async fn fetch_txt_records(resolver: &TokioAsyncResolver, domain: String) -> Result<TxtLookup, SpfFetchError> {
  match resolver.txt_lookup(domain).await {
    Err(_) => Err(SpfFetchError::NoTxtRecords),
    Ok(txt_record) => Ok(txt_record)
  }
}

fn select_spf_record(txt_records: TxtLookup) -> Result<Spf, SpfFetchError> {
  for record in txt_records.iter() {
    if record.to_string().starts_with("v=spf1") {
      let record_result: Result<Spf, SpfError>  = clean_up_record_whitespace(record.to_string())
        .parse();
      return match record_result {
        Ok(record) => Ok(record),
        Err(error) => Err(SpfFetchError::SpfParseError(error))
      }
    }
  }
  return Err(SpfFetchError::NoSpfRecords)
}

#[derive(Debug)]
pub enum SpfFetchError {
  NoTxtRecords,
  NoSpfRecords,
  SpfParseError(SpfError)
}

fn clean_up_record_whitespace(txt_record: String) -> String {
  let re = Regex::new(r"\s+").unwrap();
  let text = re.replace_all(&txt_record, " ");
  text.to_string()
}