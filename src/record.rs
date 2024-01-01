use crate::error::DnsError;
use crate::rr_fields::Type;
use crate::util::decode_dns_name;
use std::io::{Cursor, Read};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct DnsRecord {
    pub name: String,
    pub rtype: Type,
    pub class: u16,
    pub ttl: u32,
    pub data: String,
}
macro_rules! cursor_read_num {
    ($reader: expr, $buf: expr, $num_parser: path) => {{
        $reader
            .read_exact(&mut $buf)
            .map_err(|_| DnsError::DecodeError("Failed to read exact bytes from cursor"))?;
        $num_parser($buf)
    }};
}

impl DnsRecord {
    pub fn from_bytes(reader: &mut Cursor<&[u8]>) -> Result<Self, DnsError> {
        let mut buf_16 = [0u8; 2];
        let mut buf_32 = [0u8; 4];
        let name = decode_dns_name(reader)?;
        let rtype_raw = cursor_read_num!(reader, buf_16, u16::from_be_bytes);
        let rtype = Type::try_from(rtype_raw)
            .map_err(|_| DnsError::DecodeError("Failed decoding record type"))?;
        let class = cursor_read_num!(reader, buf_16, u16::from_be_bytes);
        let ttl = cursor_read_num!(reader, buf_32, u32::from_be_bytes);
        let data = Self::data_from_bytes(reader, rtype)?;
        Ok(Self {
            name,
            rtype,
            class,
            ttl,
            data,
        })
    }

    pub fn data_from_bytes(reader: &mut Cursor<&[u8]>, rtype: Type) -> Result<String, DnsError> {
        let mut buf_16 = [0u8; 2];
        let data_size = cursor_read_num!(reader, buf_16, u16::from_be_bytes);
        match rtype {
            Type::A => {
                assert!(data_size == 4);
                // pretty-print Type::A records which contain IP addresses
                let mut data = [0; 4];
                reader
                    .read_exact(&mut data)
                    .map_err(|_| DnsError::DecodeError("Failed DNS record data"))?;
                let addr = Ipv4Addr::from(data);
                Ok(addr.to_string())
            }
            Type::NS => decode_dns_name(reader),
            Type::AAAA => {
                assert!(data_size == 16);
                // pretty-print Type::AAAA records which contain IPv6 addresses
                let mut data = [0; 16];
                reader
                    .read_exact(&mut data)
                    .map_err(|_| DnsError::DecodeError("Failed DNS record data"))?;
                let addr = Ipv6Addr::from(data);
                Ok(addr.to_string())
            }
            Type::CNAME => decode_dns_name(reader),
            Type::MX | Type::TXT => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rr_fields::Class;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_record_aaaa() {
        let packet_hex = "a15e818000010001000000020377777706676f6f676c6503636f6d0000410001c00c00\
        41000100001bb6000d00010000010006026832026833c00c000100010000005200048efa5024c00c001c0001000000\
        5100102607f8b04006080d0000000000002004";
        let packet_bytes = hex::decode(packet_hex).unwrap();
        let mut reader = Cursor::new(packet_bytes.as_slice());
        let record_position = 0x49;
        reader.set_position(record_position);
        let expected = Ok(DnsRecord {
            name: "www.google.com".to_string(),
            rtype: Type::AAAA,
            class: Class::CLASS_IN as u16,
            ttl: 81,
            data: "2607:f8b0:4006:80d::2004".to_string(),
        });
        let record = DnsRecord::from_bytes(&mut reader);
        assert_eq!(record, expected)
    }
    #[test]
    fn test_record_cname() {
        let packet_hex = "8bb58180000100020000000002616106676f6f676c6503636f6d0000010001c00c00\
        0500010000009100090477777733016cc00fc02b000100010000004e00048efb286e";
        let packet_bytes = hex::decode(packet_hex).unwrap();
        let mut reader = Cursor::new(packet_bytes.as_slice());
        let record_position = 0x1f;
        reader.set_position(record_position);
        let expected = Ok(DnsRecord {
            name: "aa.google.com".to_string(),
            rtype: Type::CNAME,
            class: Class::CLASS_IN as u16,
            ttl: 145,
            data: "www3.l.google.com".to_string(),
        });
        let record = DnsRecord::from_bytes(&mut reader);
        assert_eq!(record, expected)
    }
}
