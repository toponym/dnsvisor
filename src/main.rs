use clap::Command;
use dnsvisor::resolver::Resolver;
use dnsvisor::rr_fields::Type;
use std::io::{stdin, stdout, Write};
use std::process::exit;

fn interactive() {
    let mut resolver = Resolver::new();
    loop {
        print!("Enter a domain> ");
        stdout().flush().unwrap_or_else(|_| {
            eprintln!("Error flushing output");
            exit(1);
        });
        // Read
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Error reading domain");
                exit(1);
            }
        }
        let domain_name = line.trim();
        // handle CTRL-D
        if domain_name.is_empty() {
            exit(0)
        }
        match resolver.resolve(domain_name, Type::A) {
            Ok(ip) => println!("Domain IP: {}", ip),
            Err(err) => println!("Resolver failed with error: {:?}", err),
        }
    }
}

fn main() {
    env_logger::builder().format_timestamp(None).init();
    let cmd = Command::new("dnsvisor")
        .about("DNS resolver")
        .subcommand_required(true)
        .subcommand(Command::new("interactive").about("Interactive prompt to look up DNS records"));
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("interactive", _matches)) => interactive(),
        _ => {
            eprintln!("Error: invalid arguments passed");
            exit(1);
        }
    }
}
