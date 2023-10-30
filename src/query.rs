use crate::header::DnsHeader;
use crate::packet::DnsPacket;
use crate::question::DnsQuestion;
use crate::rr_fields::{Class, Type};

use rand::random;
use std::net::UdpSocket;

pub fn build_query(domain_name: &str, record_type: Type) -> Vec<u8> {
    let id: u16 = random();
    let no_recursion = 0;
    let header = DnsHeader {
        id,
        flags: no_recursion,
        num_questions: 1,
        num_answers: 0,
        num_authorities: 0,
        num_additionals: 0,
    };
    let question = DnsQuestion::new(domain_name, record_type, Class::CLASS_IN);
    let mut query_bytes = header.to_bytes();
    query_bytes.append(&mut question.to_bytes());
    query_bytes
}

pub fn send_query(nameserver: &str, domain_name: &str, record_type: Type) -> DnsPacket {
    // TODO different buf size?
    let mut buf: [u8; 1024] = [0; 1024];
    let query = build_query(domain_name, record_type);
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let _res = socket.send_to(&query, (nameserver, 53)).unwrap();
    let (_num_bytes, _src_addr) = socket.recv_from(&mut buf).unwrap();
    DnsPacket::from_bytes(&buf)
}
