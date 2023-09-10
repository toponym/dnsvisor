use crate::rr_fields::{Class, Type};
use crate::util;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use hex;
    use pretty_assertions::assert_eq;
    use std::io::Cursor;
    #[test]
    fn question_sample_response() {
        let response = "60568180000100010000000003777777076578616d706c6503636f6d0000010001c00c000100010000529b00045db8d822";
        let response_bytes = hex::decode(response).unwrap();
        let mut reader = Cursor::new(response_bytes.as_slice());
        let start_pos = 12;
        reader.set_position(start_pos);
        let header = DnsQuestion::from_bytes(&mut reader);
        let expected = DnsQuestion {
            name: String::from("www.example.com"),
            qtype: 1,
            class: 1,
        };
        let expected_end_pos = 33;
        assert_eq!(header, expected);
        assert_eq!(reader.position(), expected_end_pos);
    }
}
