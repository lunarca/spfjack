use trust_dns_resolver::Resolver;

pub fn domain_registered(resolver: Resolver, domain: String) -> bool {
    match resolver.ipv4_lookup(domain) {
        Ok(_) => true,
        Err(_) => false
    }
}