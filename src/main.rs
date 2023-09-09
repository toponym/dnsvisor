use dnsvisor::{self, rr_fields::Type};

fn main() {
    let query = dnsvisor::query::build_query(String::from("www.example.com"), Type::A);
    dnsvisor::socket::query_google(&query);
}
