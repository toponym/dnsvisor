use crate::error::DnsError;
use crate::rr_fields::{Class, Type};
use crate::util;
use std::io::Cursor;
use std::io::Read;
#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: Type,
    pub class: Class,
}

impl DnsQuestion {
    pub fn new(domain_name: &str, record_type: Type, class: Class) -> Self {
        Self {
            name: domain_name.to_string(),
            qtype: record_type,
            class,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = util::encode_dns_name(&self.name);
        bytes.extend_from_slice(&(self.qtype as u16).to_be_bytes());
        bytes.extend_from_slice(&(self.class as u16).to_be_bytes());
        bytes
    }

    pub fn from_bytes(reader: &mut Cursor<&[u8]>) -> Result<DnsQuestion, DnsError> {
        let mut bytes = [0u8, 2];
        let name = util::decode_dns_name(reader)?;
        reader
            .read_exact(&mut bytes)
            .map_err(|_| DnsError::DecodeError("Failed to decode DNS question".to_string()))?;
        let qtype = Type::try_from(u16::from_be_bytes(bytes))?;
        reader
            .read_exact(&mut bytes)
            .map_err(|_| DnsError::DecodeError("Failed to decode DNS question".to_string()))?;
        let class = Class::try_from(u16::from_be_bytes(bytes))?;
        Ok(Self { name, qtype, class })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;
    use pretty_assertions::assert_eq;
    use std::io::Cursor;
    #[test]
    fn question_from_bytes() {
        let response = "60568180000100010000000003777777076578616d706c6503636f6d0000010001c00c000100010000529b00045db8d822";
        let response_bytes = hex::decode(response).unwrap();
        let mut reader = Cursor::new(response_bytes.as_slice());
        let start_pos = 12;
        reader.set_position(start_pos);
        let header = DnsQuestion::from_bytes(&mut reader).unwrap();
        let expected = DnsQuestion {
            name: String::from("www.example.com"),
            qtype: Type::A,
            class: Class::CLASS_IN,
        };
        let expected_end_pos = 33;
        assert_eq!(header, expected);
        assert_eq!(reader.position(), expected_end_pos);
    }
    #[test]
    fn question_to_bytes_sample() {
        let question = DnsQuestion {
            name: String::from("www.example.com"),
            qtype: Type::A,
            class: Class::CLASS_IN,
        };
        let expected_str = "03777777076578616d706c6503636f6d0000010001";
        let expected = hex::decode(expected_str).unwrap();
        let result = question.to_bytes();
        assert_eq!(result, expected);
    }
}
