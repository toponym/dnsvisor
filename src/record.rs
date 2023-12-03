use crate::error::DnsError;
use crate::rr_fields::Type;
use crate::util::decode_dns_name;
use std::io::{Cursor, Read};
use std::net::{Ipv4Addr, Ipv6Addr};

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
