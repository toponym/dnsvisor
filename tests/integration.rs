use dnsvisor::resolve;
use dnsvisor::rr_fields::Type;

#[cfg(test)]
#[test]
fn resolve_facebook() {
    let domain_name = "www.facebook.com";
    let res = resolve(domain_name, Type::A);
    assert!(res.is_ok())
}

#[cfg(test)]
#[test]
fn resolve_twitter() {
    let domain_name = "twitter.com";
    let res = resolve(domain_name, Type::A);
    assert!(res.is_ok())
}
