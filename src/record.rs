use crate::rr_fields::Type;
use crate::util::decode_dns_name;
use std::io::{Cursor, Read};
#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsRecord {
    name: String,
    pub rtype: Type,
    class: u16,
    ttl: u32,
    pub data: String,
}
macro_rules! cursor_read_num {
    ($reader: expr, $buf: expr, $num_parser: path) => {{
        $reader.read_exact(&mut $buf).unwrap();
        $num_parser($buf)
    }};
}

impl DnsRecord {
    pub fn from_bytes(reader: &mut Cursor<&[u8]>) -> Self {
        let mut buf_16 = [0u8; 2];
        let mut buf_32 = [0u8; 4];
        let name = decode_dns_name(reader);
        let rtype_raw = cursor_read_num!(reader, buf_16, u16::from_be_bytes);
        let rtype = Type::from(rtype_raw);
        let class = cursor_read_num!(reader, buf_16, u16::from_be_bytes);
        let ttl = cursor_read_num!(reader, buf_32, u32::from_be_bytes);
        let data = Self::data_from_bytes(reader, rtype);
        Self {
            name,
            rtype,
            class,
            ttl,
            data,
        }
    }

    pub fn data_from_bytes(reader: &mut Cursor<&[u8]>, rtype: Type) -> String {
        let mut buf_16 = [0u8; 2];
        let data_size = cursor_read_num!(reader, buf_16, u16::from_be_bytes);
        match rtype {
            Type::A => {
                // pretty-print Type::A records which contain IP addresses
                let mut data = vec![0; data_size as usize];
                reader.read_exact(&mut data).unwrap();
                assert!(data.len() == 4);
                let converted: Vec<String> = data.iter().map(|x| x.to_string()).collect();
                converted.join(".")
            }
            Type::NS => {
                decode_dns_name(reader)
            }
            Type::CNAME | Type::MX | Type::TXT | Type::AAAA => todo!(),
        }
    }
}
