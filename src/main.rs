#![warn(clippy::unwrap_used, clippy::panic)]
use clap::{Arg, Command};
use dnsvisor::packet::DnsPacket;
use dnsvisor::resolver::Resolver;
use dnsvisor::rr_fields::Type;
use log::{debug, error, warn};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, stdin, stdout, BufRead, BufReader, Write};
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use tokio::net::UdpSocket;

async fn interactive() {
    let mut resolver = Resolver::default();
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
        match resolver.resolve(domain_name, Type::A).await {
            Ok(ip) => println!("Domain IP: {}", ip),
            Err(err) => println!("Resolver failed with error: {:?}", err),
        }
    }
}

async fn handle_request(mut resolver: Resolver, socket: Arc<UdpSocket>) {
    // Per RFC 1035 the max size for UDP messages is 512 bytes
    let mut buf = [0u8; 512];
    let (_n_bytes, src_addr) = match socket.recv_from(&mut buf).await {
        Ok((n_bytes, src_addr)) => (n_bytes, src_addr),
        Err(_) => {
            error!("Failed to receive request from socket");
            return;
        }
    };
    debug!("Received request from {:?}", src_addr);
    let query_packet = match DnsPacket::from_bytes(&buf[..]) {
        Ok(packet) => packet,
        Err(err) => {
            error!(
                "Failed to decode request packet with error {:?}. Skipping.",
                err
            );
            return;
        }
    };
    let response_res = resolver.resolve_packet(query_packet.clone()).await;
    match response_res {
        Ok(response_packet) => send_response(response_packet, &src_addr, &socket).await,
        Err(err) => {
            error!(
                "Resolver failed with error {:?}. Sending error response.",
                err
            );
            let err_packet = query_packet.make_error_response(err);
            send_response(err_packet, &src_addr, &socket).await
        }
    }
}

async fn server(ip: &IpAddr, port: &u16, blocklist_option: Option<&PathBuf>) {
    let blocklist = build_blocklist(blocklist_option).unwrap_or_else(|_| {
        eprintln!("Failed to read blocklist");
        exit(1);
    });
    let resolver = Resolver::new(blocklist);
    let addr = SocketAddr::from((*ip, *port));
    let socket = Arc::new(UdpSocket::bind(addr).await.unwrap_or_else(|_| {
        eprintln!("Failed to bind to socket");
        exit(1);
    }));
    debug!("Server listening on {:?}", socket);
    loop {
        let resolver_clone = Resolver {
            cache: resolver.cache.clone(),
            blocklist: resolver.blocklist.clone(),
        };
        let socket_clone = socket.clone();
        tokio::spawn(async move { handle_request(resolver_clone, socket_clone).await });
    }
}

fn build_blocklist(blocklist_option: Option<&PathBuf>) -> io::Result<HashSet<String>> {
    let mut blocklist = HashSet::new();
    if let Some(blocklist_path) = blocklist_option {
        let file = File::open(blocklist_path)?;
        let reader = BufReader::new(file);
        for line_result in reader.lines() {
            let line = line_result?;
            // skip commented lines
            if !line.starts_with('#') {
                blocklist.insert(line);
            }
        }
    }
    Ok(blocklist)
}

async fn send_response(packet: DnsPacket, src_addr: &SocketAddr, socket: &UdpSocket) {
    debug!("Sending response to {:?}", src_addr);
    match packet.to_bytes() {
        Ok(bytes) => {
            if let Err(err) = socket.send_to(&bytes, src_addr).await {
                error!("Failed to send response with error: {:?}. Skipping.", err)
            }
        }
        Err(err) => error!(
            "Failed to encode the response with error: {:?}. Skipping.",
            err
        ),
    }
}

macro_rules! exit_invalid_args {
    () => {{
        eprintln!("Error: invalid arguments passed");
        exit(1);
    }};
}

#[tokio::main]
async fn main() {
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
                )
                .arg(
                    Arg::new("blocklist")
                        .short('b')
                        .long("blocklist")
                        .help("File with list of domains to blocklist")
                        .value_name("FILE")
                        .required(false)
                        .value_parser(clap::value_parser!(PathBuf)),
                ),
        );
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("interactive", _matches)) => interactive().await,
        Some(("server", matches)) => {
            let ip_address = matches
                .get_one::<IpAddr>("ip_address")
                .unwrap_or_else(|| exit_invalid_args!());
            let port = matches
                .get_one::<u16>("port")
                .unwrap_or_else(|| exit_invalid_args!());
            let blocklist_option = matches.get_one::<PathBuf>("blocklist");
            server(ip_address, port, blocklist_option).await;
        }
        _ => exit_invalid_args!(),
    }
}
