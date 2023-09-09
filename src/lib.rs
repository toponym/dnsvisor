use rr_fields::Type;
// TODO visibility
pub mod header;
pub mod packet;
pub mod query;
pub mod question;
pub mod record;
pub mod rr_fields;
pub mod util;

pub fn resolve(domain_name: &str) -> String {
    let google_nameserver = "8.8.8.8:53";
    // TODO make real resolver
    let res = query::send_query(google_nameserver, domain_name, Type::A);
    format!("{:?}", res)
}
