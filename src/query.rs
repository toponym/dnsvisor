use crate::header::DnsHeader;
use crate::packet::DnsPacket;
use crate::question::DnsQuestion;
use crate::rr_fields::{Class, Type};

use rand::random;
use std::io::Cursor;
use std::io::Read;
use std::net::UdpSocket;

pub fn encode_dns_name(domain_name: &str) -> Vec<u8> {
    assert!(domain_name.is_ascii());
    let mut encoded: Vec<u8> = vec![];
    for part in domain_name.split('.') {
        assert!(part.len() <= 63);
        let length = part.len() as u8;
        encoded.push(length);
        encoded.extend_from_slice(part.as_bytes());
    }
    encoded.push(0);
    encoded
}

fn decode_dns_name(reader: &mut Cursor<&[u8]>) -> String {
    let mut parts: Vec<String> = vec![];
    let mut part: [u8; 63] = [0; 63];
    let mut length: [u8; 1] = [0; 1];
    reader.read_exact(&mut length).unwrap();
    while length[0] != 0 {
        if length[0] == 0b1100_0000 {
            parts.push(decode_compressed_name(length[0], reader));
            break;
        } else {
            reader.take(length[0] as u64).read_exact(&mut part).unwrap();
            parts.push(String::from_utf8(part.to_vec()).unwrap());
            part.iter_mut().for_each(|x| *x = 0);
        }
        reader.read_exact(&mut length).unwrap();
    }
    parts.join(".")
}

fn decode_compressed_name(length: u8, reader: &mut Cursor<&[u8]>) -> String {
    let mut byte: [u8; 1] = [0; 1];
    reader.read_exact(&mut byte).unwrap();
    let pointer: u64 = (length & 0b0011_1111) as u64 + byte[0] as u64;
    let prev_position = reader.position();
    reader.set_position(pointer);
    let res = decode_dns_name(reader);
    reader.set_position(prev_position);
    res
}

pub fn build_query(domain_name: &str, record_type: Type) -> Vec<u8> {
    let id: u16 = random();
    let recursion_desired = 1 << 8;
    let header = DnsHeader {
        id,
        flags: recursion_desired,
        num_questions: 1,
        num_answers: 0,
        num_authorities: 0,
        num_additionals: 0,
    };
    let question = DnsQuestion {
        name: encode_dns_name(domain_name),
        qtype: record_type as u16,
        class: Class::CLASS_IN as u16,
    };
    let mut query_bytes = header.to_bytes();
    query_bytes.append(&mut question.to_bytes());
    query_bytes
}

fn send_query(nameserver: &str, domain_name: &str, record_type: Type) -> DnsPacket {
    // TODO different buf size?
    let mut buf: [u8; 1024] = [0; 1024];
    let query = build_query(domain_name, record_type);
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let _res = socket.send_to(&query, nameserver).unwrap();
    let (num_bytes, src_addr) = socket.recv_from(&mut buf).unwrap();
    println!("Received {} bytes from {}", num_bytes, src_addr);
    println!("Message: {:x?}", buf);
    DnsPacket::from_bytes(&buf)
}
