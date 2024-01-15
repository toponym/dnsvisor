use clap::{Arg, Command};
use dnsvisor::packet::DnsPacket;
use dnsvisor::resolver::Resolver;
use dnsvisor::rr_fields::Type;
use log::debug;
use std::io::{stdin, stdout, Write};
use std::net::{IpAddr, SocketAddr, UdpSocket};
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

fn server(ip: &IpAddr, port: &u16) {
    // TODO remove unwrap and send error packet to client on error
    let mut resolver = Resolver::new();
    let addr = SocketAddr::from((*ip, *port));
    let socket = UdpSocket::bind(addr).unwrap();
    debug!("Server listening on {:?}", socket);
    loop {
        // Per RFC 1035 the max size for UDP messages is 512 bytes
        let mut buf = [0u8; 512];
        let (_n_bytes, src_addr) = socket.recv_from(&mut buf).unwrap();
        debug!("Received request from {:?}", src_addr);
        let query_packet = DnsPacket::from_bytes(&buf[..]).unwrap();
        let response_packet = resolver.resolve_packet(query_packet).unwrap();
        let response_bytes = response_packet.to_bytes().unwrap();
        debug!("Sending response to {:?}", src_addr);
        socket.send_to(&response_bytes, src_addr).unwrap();
    }
}

macro_rules! exit_invalid_args {
    () => {{
        eprintln!("Error: invalid arguments passed");
        exit(1);
    }};
}

fn main() {
    env_logger::builder().format_timestamp(None).init();
    let cmd = Command::new("dnsvisor")
        .about("DNS resolver")
        .subcommand_required(true)
        .subcommand(Command::new("interactive").about("Interactive prompt to look up DNS records"))
        .subcommand(
            Command::new("server")
                .about("UDP server to respond to DNS Requests")
                .arg(
                    Arg::new("ip_address")
                        .help("Server IP Address")
                        .required(true)
                        .value_parser(clap::value_parser!(IpAddr)),
                )
                .arg(
                    Arg::new("port")
                        .help("Server Port")
                        .required(true)
                        .value_parser(clap::value_parser!(u16)),
                ),
        );
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("interactive", _matches)) => interactive(),
        Some(("server", matches)) => {
            let ip_address = matches
                .get_one::<IpAddr>("ip_address")
                .unwrap_or_else(|| exit_invalid_args!());
            let port = matches
                .get_one::<u16>("port")
                .unwrap_or_else(|| exit_invalid_args!());
            server(ip_address, port);
        }
        _ => exit_invalid_args!(),
    }
}
