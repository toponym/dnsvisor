use crate::cursor_read_num;
use crate::error::DnsError;
use crate::question::DnsQuestion;
use crate::rr_fields::{Class, Type};
use crate::util::{decode_dns_name, encode_dns_name};
use std::io::{Cursor, Read};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub struct DnsRecord {
    pub name: String,
    pub class: Class,
    pub ttl: u32,
    pub rdata: Rdata,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum Rdata {
    A(String),
    NS(String),
    CNAME(String),
    AAAA(String),
    SOA(RdataSOA),
    MX(RdataMX),
}

impl DnsRecord {
    pub fn get_type(&self) -> Type {
        match self.rdata {
            Rdata::A(_) => Type::A,
            Rdata::NS(_) => Type::NS,
            Rdata::CNAME(_) => Type::CNAME,
            Rdata::AAAA(_) => Type::AAAA,
            Rdata::SOA(_) => Type::SOA,
            Rdata::MX(_) => Type::MX,
        }
    }

    pub fn from_bytes(reader: &mut Cursor<&[u8]>) -> Result<Self, DnsError> {
        let mut buf_16 = [0u8; 2];
        let mut buf_32 = [0u8; 4];
        let name = decode_dns_name(reader)?;
        let rtype_raw = cursor_read_num!(reader, buf_16, u16::from_be_bytes);
        let rtype = Type::try_from(rtype_raw)?;
        let class_raw = cursor_read_num!(reader, buf_16, u16::from_be_bytes);
        let class = Class::try_from(class_raw)?;
        let ttl = cursor_read_num!(reader, buf_32, u32::from_be_bytes);
        let rdata = Self::data_from_bytes(reader, rtype)?;
        Ok(Self {
            name,
            class,
            ttl,
            rdata,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, DnsError> {
        let mut bytes: Vec<u8> = vec![];
        bytes.append(&mut encode_dns_name(&self.name));
        bytes.extend_from_slice(&u16::to_be_bytes(self.get_type() as u16));
        bytes.extend_from_slice(&u16::to_be_bytes(self.class as u16));
        bytes.extend_from_slice(&u32::to_be_bytes(self.ttl));
        let data = self.data_to_bytes()?;
        let data_size = u16::try_from(data.len())
            .map_err(|_| DnsError::EncodeError("Data size exceeds u16 limit"))?;
        bytes.extend_from_slice(&u16::to_be_bytes(data_size));
        bytes.extend(data);
        Ok(bytes)
    }

    pub fn data_from_bytes(reader: &mut Cursor<&[u8]>, rtype: Type) -> Result<Rdata, DnsError> {
        let mut buf_16 = [0u8; 2];
        let data_size = cursor_read_num!(reader, buf_16, u16::from_be_bytes);
        match rtype {
            Type::A => {
                assert!(data_size == 4);
                // pretty-print Type::A records which contain IP addresses
                let mut data = [0; 4];
                reader
                    .read_exact(&mut data)
                    .map_err(|_| DnsError::DecodeError("Failed DNS record data".to_string()))?;
                let addr = Ipv4Addr::from(data);
                Ok(Rdata::A(addr.to_string()))
            }
            Type::NS => Ok(Rdata::NS(decode_dns_name(reader)?)),
            Type::AAAA => {
                assert!(data_size == 16);
                // pretty-print Type::AAAA records which contain IPv6 addresses
                let mut data = [0; 16];
                reader
                    .read_exact(&mut data)
                    .map_err(|_| DnsError::DecodeError("Failed DNS record data".to_string()))?;
                let addr = Ipv6Addr::from(data);
                Ok(Rdata::AAAA(addr.to_string()))
            }
            Type::CNAME => Ok(Rdata::CNAME(decode_dns_name(reader)?)),
            Type::SOA => Ok(Rdata::SOA(RdataSOA::from_bytes(reader)?)),
            Type::MX => Ok(Rdata::MX(RdataMX::from_bytes(reader)?)),
            _ => Err(DnsError::NotImplementedError(format!(
                "Decoding data for type {:?} not supported yet",
                rtype
            ))),
        }
    }

    pub fn data_to_bytes(&self) -> Result<Vec<u8>, DnsError> {
        match &self.rdata {
            Rdata::A(string) => {
                let addr = string.parse::<Ipv4Addr>().map_err(|_| {
                    DnsError::EncodeError("Failed encoding Ipv4Addr in Type A record")
                })?;
                Ok(u32::from(addr).to_be_bytes().to_vec())
            }
            Rdata::NS(string) => Ok(encode_dns_name(string)),
            Rdata::AAAA(string) => {
                let addr = string.parse::<Ipv6Addr>().map_err(|_| {
                    DnsError::EncodeError("Failed encoding Ipv4Addr in Type A record")
                })?;
                Ok(u128::from(addr).to_be_bytes().to_vec())
            }
            Rdata::CNAME(string) => Ok(encode_dns_name(string)),
            Rdata::SOA(rdata_soa) => rdata_soa.to_bytes(),
            Rdata::MX(rdata_mx) => rdata_mx.to_bytes(),
        }
    }

    pub fn get_question(&self) -> DnsQuestion {
        DnsQuestion {
            name: self.name.clone(),
            qtype: self.get_type(),
            class: self.class,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RdataSOA {
    pub mname: String,
    rname: String,
    serial: u32,
    refresh: u32,
    retry: u32,
    expire: u32,
}

impl RdataSOA {
    pub fn from_bytes(reader: &mut Cursor<&[u8]>) -> Result<Self, DnsError> {
        let mut buf_32 = [0u8; 4];
        let mname = decode_dns_name(reader)?;
        let rname = decode_dns_name(reader)?;
        let serial = cursor_read_num!(reader, buf_32, u32::from_be_bytes);
        let refresh = cursor_read_num!(reader, buf_32, u32::from_be_bytes);
        let retry = cursor_read_num!(reader, buf_32, u32::from_be_bytes);
        let expire = cursor_read_num!(reader, buf_32, u32::from_be_bytes);
        Ok(Self {
            mname,
            rname,
            serial,
            refresh,
            retry,
            expire,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, DnsError> {
        let mut bytes: Vec<u8> = vec![];
        bytes.append(&mut encode_dns_name(&self.mname));
        bytes.append(&mut encode_dns_name(&self.rname));
        bytes.extend_from_slice(&u32::to_be_bytes(self.serial));
        bytes.extend_from_slice(&u32::to_be_bytes(self.refresh));
        bytes.extend_from_slice(&u32::to_be_bytes(self.retry));
        bytes.extend_from_slice(&u32::to_be_bytes(self.expire));
        Ok(bytes)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RdataMX {
    preference: u16,
    pub exchange: String,
}

impl RdataMX {
    pub fn from_bytes(reader: &mut Cursor<&[u8]>) -> Result<Self, DnsError> {
        let mut buf_16 = [0u8; 2];
        let preference = cursor_read_num!(reader, buf_16, u16::from_be_bytes);
        let exchange = decode_dns_name(reader)?;
        Ok(Self {
            preference,
            exchange,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, DnsError> {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend_from_slice(&u16::to_be_bytes(self.preference));
        bytes.append(&mut encode_dns_name(&self.exchange));
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rr_fields::Class;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_from_bytes_record_aaaa() {
        let packet_hex = "a15e818000010001000000020377777706676f6f676c6503636f6d0000410001c00c00\
        41000100001bb6000d00010000010006026832026833c00c000100010000005200048efa5024c00c001c0001000000\
        5100102607f8b04006080d0000000000002004";
        let packet_bytes = hex::decode(packet_hex).unwrap();
        let mut reader = Cursor::new(packet_bytes.as_slice());
        let record_position = 0x49;
        reader.set_position(record_position);
        let expected = Ok(DnsRecord {
            name: "www.google.com".to_string(),
            class: Class::CLASS_IN,
            ttl: 81,
            rdata: Rdata::AAAA("2607:f8b0:4006:80d::2004".to_string()),
        });
        let record = DnsRecord::from_bytes(&mut reader);
        assert_eq!(record, expected)
    }
    #[test]
    fn test_from_bytes_record_cname() {
        let packet_hex = "8bb58180000100020000000002616106676f6f676c6503636f6d0000010001c00c00\
        0500010000009100090477777733016cc00fc02b000100010000004e00048efb286e";
        let packet_bytes = hex::decode(packet_hex).unwrap();
        let mut reader = Cursor::new(packet_bytes.as_slice());
        let record_position = 0x1f;
        reader.set_position(record_position);
        let expected = Ok(DnsRecord {
            name: "aa.google.com".to_string(),
            class: Class::CLASS_IN,
            ttl: 145,
            rdata: Rdata::CNAME("www3.l.google.com".to_string()),
        });
        let record = DnsRecord::from_bytes(&mut reader);
        assert_eq!(record, expected)
    }
    #[test]
    fn test_to_bytes_record_aaaa() {
        let expected_str = "0377777706676f6f676c6503636f6d00001c0001000000\
        5100102607f8b04006080d0000000000002004";
        let expected = hex::decode(expected_str).unwrap();
        let record = DnsRecord {
            name: "www.google.com".to_string(),
            class: Class::CLASS_IN,
            ttl: 81,
            rdata: Rdata::AAAA("2607:f8b0:4006:80d::2004".to_string()),
        };
        let result = record.to_bytes().unwrap();
        assert_eq!(result, expected)
    }
    #[test]
    fn test_data_to_bytes_aaaa() {
        let record = DnsRecord {
            name: "www.google.com".to_string(),
            class: Class::CLASS_IN,
            ttl: 81,
            rdata: Rdata::AAAA("2607:f8b0:4006:80d::2004".to_string()),
        };
        let expected_str = "2607f8b04006080d0000000000002004".to_string();
        let expected = hex::decode(expected_str).unwrap();
        let result = record.data_to_bytes().unwrap();
        assert_eq!(result, expected);
    }
}
