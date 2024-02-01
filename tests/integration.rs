use dnsvisor::resolver::Resolver;
use dnsvisor::rr_fields::Type;

#[cfg(test)]
#[test]
fn resolve_facebook() {
    let domain_name = "www.facebook.com";
    let mut resolver = Resolver::default();
    let res = resolver.resolve(domain_name, Type::A);
    assert!(res.is_ok())
}

#[cfg(test)]
#[test]
fn resolve_twitter() {
    let domain_name = "twitter.com";
    let mut resolver = Resolver::default();
    let res = resolver.resolve(domain_name, Type::A);
    assert!(res.is_ok())
}
