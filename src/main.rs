mod spf;

fn main() {
    let domain = "bishopfox.com.";

    let spf_record = spf::fetch_and_parse(domain);
    println!("SPF Record: {:?}", spf_record);
}
