/// Resolve a single domain from the command line
use dnsvisor::resolver::Resolver;
use dnsvisor::rr_fields::Type;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let domain_name = &args[1];
    env_logger::builder().format_timestamp(None).init();
    println!("Looking up domain: {}", domain_name);
    let mut resolver = Resolver::default();
    match resolver.resolve(domain_name, Type::A) {
        Ok(ip) => {
            println!("Domain IP: {}", ip);
            std::process::exit(0);
        }
        Err(err) => {
            println!("Failed to resolve with error: {:?}", err);
            process::exit(1);
        }
    }
}
