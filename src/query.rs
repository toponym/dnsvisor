use crate::rr_fields::{Class, Type};
use bincode::Options;
use rand::random;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct DnsHeader {
    id: u16,
    flags: u16,
    num_questions: u16,
    num_answers: u16,
    num_authorities: u16,
    num_additionals: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct DnsQuestion {
    name: Vec<u8>,
    qtype: u16,
    class: u16,
}

impl DnsQuestion {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.name.clone();
        bytes.extend_from_slice(&self.qtype.to_be_bytes());
        bytes.extend_from_slice(&self.class.to_be_bytes());
        bytes
    }
}

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

pub fn build_query(domain_name: String, record_type: Type) -> Vec<u8> {
    //let name = encode_dns_name(domain_name);
    let id: u16 = random();
    let recursion_desired = 1 << 8;
    let options = bincode::DefaultOptions::new()
        .with_big_endian()
        .with_fixint_encoding();
    let header = DnsHeader {
        id,
        flags: recursion_desired,
        num_questions: 1,
        num_answers: 0,
        num_authorities: 0,
        num_additionals: 0,
    };
    let question = DnsQuestion {
        name: encode_dns_name(&domain_name),
        qtype: record_type as u16,
        class: Class::CLASS_IN as u16,
    };
    let mut query_bytes = options.serialize(&header).unwrap();
    query_bytes.append(&mut question.to_bytes());
    query_bytes
}
