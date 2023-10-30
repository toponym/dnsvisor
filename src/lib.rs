use log::{debug, info};
use rr_fields::Type;
// TODO visibility
pub mod header;
pub mod packet;
pub mod query;
pub mod question;
pub mod record;
pub mod rr_fields;
pub mod util;

pub fn resolve(domain_name: &str, record_type: Type) -> String {
    // Verisign root nameserver
    let root_nameserver = String::from("198.41.0.4");
    let mut nameserver = root_nameserver;
    loop {
        info!("Querying {} for {}", nameserver, domain_name);
        let response = query::send_query(&nameserver, domain_name, record_type);
        if let Some(ip) = response.get_answer() {
            debug!("Got ip: {}", ip);
            return ip;
        } else if let Some(ns_ip) = response.get_nameserver_ip() {
            debug!("Got nameserver ip: {}", ns_ip);
            nameserver = ns_ip;
        } else if let Some(ns_domain) = response.get_nameserver() {
            debug!("Got nameserver domain: {}", ns_domain);
            nameserver = resolve(&ns_domain, Type::A);
        } else {
            panic!("Unexpected response: {:?}", response);
        }
    }
}
