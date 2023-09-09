use crate::rr_fields::{Class, Type};
use crate::util;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: u16,
    pub class: u16,
}

impl DnsQuestion {
    pub fn new(domain_name: &str, record_type: Type, class: Class) -> Self {
        Self {
            name: domain_name.to_string(),
            qtype: record_type as u16,
            class: class as u16,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = util::encode_dns_name(&self.name);
        bytes.extend_from_slice(&self.qtype.to_be_bytes());
        bytes.extend_from_slice(&self.class.to_be_bytes());
        bytes
    }

    pub fn from_bytes(reader: &mut Cursor<&[u8]>) -> DnsQuestion {
        let mut bytes = [0u8, 2];
        let name = util::decode_dns_name(reader);
        reader.read_exact(&mut bytes).unwrap();
        let qtype = u16::from_be_bytes(bytes);
        reader.read_exact(&mut bytes).unwrap();
        let class = u16::from_be_bytes(bytes);
        Self { name, qtype, class }
    }
}
