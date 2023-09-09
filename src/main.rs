use dnsvisor::resolve;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let domain_name = &args[1];
    println!("Domain's IP: {}", resolve(domain_name));
}
