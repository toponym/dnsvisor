use std::net::SocketAddr;
// TODO visibility
pub mod header;
pub mod packet;
pub mod query;
pub mod question;
pub mod record;
pub mod rr_fields;

pub fn resolve(_domain_name: String) -> SocketAddr {
    let google_nameserver = "8.8.8.8:53";
    unimplemented!();
}
