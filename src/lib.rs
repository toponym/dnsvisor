#![warn(clippy::unwrap_used, clippy::panic, clippy::print_stdout)]
use error::DnsError;
use log::{debug, info};
use rr_fields::Type;

pub mod error;
mod header;
mod packet;
mod query;
mod question;
mod record;
pub mod rr_fields;
mod util;

pub fn resolve(req_domain_name: &str, record_type: Type) -> Result<String, DnsError> {
    // Verisign root nameserver
    let root_nameserver = String::from("198.41.0.4");
    let mut nameserver = root_nameserver;
    let mut domain_name = String::from(req_domain_name);
    loop {
        info!("Querying {} for {}", nameserver, domain_name);
        let response = query::send_query(&nameserver, &domain_name, record_type)?;
        if let Some((answer_type, answer)) = response.get_answer() {
            match answer_type {
                Type::A => {
                    debug!("Got ip: {}", answer);
                    return Ok(answer.to_string());
                }
                Type::CNAME => {
                    debug!("Got CNAME domain: {}", answer);
                    domain_name = answer.to_string();
                }
                _ => {
                    return Err(DnsError::ResolveError(format!(
                        "Unexpected answer type: {:?}",
                        answer_type
                    )))
                }
            }
        } else if let Some(ns_ip) = response.get_nameserver_ip() {
            debug!("Got nameserver ip: {}", ns_ip);
            nameserver = ns_ip.to_string();
        } else if let Some(ns_domain) = response.get_nameserver() {
            debug!("Got nameserver domain: {}", ns_domain);
            nameserver = resolve(ns_domain, Type::A)?;
        } else {
            return Err(DnsError::ResolveError(format!(
                "Unexpected response: {:?}",
                response
            )));
        }
    }
}
