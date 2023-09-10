use dnsvisor::resolve;
use dnsvisor::rr_fields::Type;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let domain_name = &args[1];
    println!("Looking up domain: {}", domain_name); 
    println!("Domain IP: {}", resolve(domain_name, Type::A));
}
