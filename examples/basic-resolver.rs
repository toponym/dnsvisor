/// Resolve a single domain from the command line
use dnsvisor::resolve;
use dnsvisor::rr_fields::Type;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let domain_name = &args[1];
    env_logger::builder().format_timestamp(None).init();
    println!("Looking up domain: {}", domain_name);
    match resolve(domain_name, Type::A) {
        Ok(ip) => println!("Domain IP: {}", ip),
        Err(err) => println!("Failed to resolve with error: {:?}", err),
    }
}
