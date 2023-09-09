use crate::util::decode_dns_name;
use std::io::{Cursor, Read};
#[derive(Debug)]
pub struct DnsRecord {
    name: String,
    rtype: u16,
    class: u16,
    ttl: u32,
    data: Vec<u8>,
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
}
