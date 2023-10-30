use crate::rr_fields::Type;
use crate::util::decode_dns_name;
use std::io::{Cursor, Read};
#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsRecord {
    name: String,
    pub rtype: u16,
    class: u16,
    ttl: u32,
    pub data: Vec<u8>,
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
        let rtype = cursor_read_num!(reader, buf_16, u16::from_be_bytes);
        let class = cursor_read_num!(reader, buf_16, u16::from_be_bytes);
        let ttl = cursor_read_num!(reader, buf_32, u32::from_be_bytes);
        let data_size = cursor_read_num!(reader, buf_16, u16::from_be_bytes);
        let mut data = vec![0; data_size as usize];
        reader.read_exact(&mut data).unwrap();
        Self {
            name,
            rtype,
            class,
            ttl,
            data,
        }
    }

    pub fn fmt_data(&self) -> String {
        let type_enum = Type::from(self.rtype);
        match type_enum {
            Type::A => {
                // pretty-print Type::A records which contain IP addresses
                assert!(self.data.len() == 4);
                let converted: Vec<String> = self.data.iter().map(|x| x.to_string()).collect();
                converted.join(".")
            }
            Type::NS => {
                let mut cursor = Cursor::new(self.data.as_slice());
                decode_dns_name(&mut cursor)
            }
        }
    }
}
