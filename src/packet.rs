use crate::error::DnsError;
use crate::header::DnsHeader;
use crate::question::DnsQuestion;
use crate::record::{DnsRecord, Rdata};
use crate::rr_fields::HeaderFlags;
use std::io::Cursor;
use std::net::UdpSocket;
use std::vec;

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub additionals: Vec<DnsRecord>,
}

macro_rules! parse_num_items {
    ($reader: expr, $num: expr, $parser: path) => {{
        let result: Result<Vec<_>, DnsError> = (0..$num).map(|_| $parser($reader)).collect();
        result?
    }};
}

impl DnsPacket {
    pub fn from_bytes(buf: &[u8]) -> Result<Self, DnsError> {
        let mut reader = Cursor::new(buf);
        let header = DnsHeader::from_bytes(&mut reader)?;
        let questions: Vec<DnsQuestion> =
            parse_num_items!(&mut reader, header.num_questions, DnsQuestion::from_bytes);
        let answers: Vec<DnsRecord> =
            parse_num_items!(&mut reader, header.num_answers, DnsRecord::from_bytes);
        let authorities: Vec<DnsRecord> =
            parse_num_items!(&mut reader, header.num_authorities, DnsRecord::from_bytes);
        let additionals: Vec<DnsRecord> =
            parse_num_items!(&mut reader, header.num_additionals, DnsRecord::from_bytes);
        Ok(Self {
            header,
            questions,
            answers,
            authorities,
            additionals,
        })
    }

    pub fn to_bytes(self) -> Result<Vec<u8>, DnsError> {
        let mut bytes = self.header.to_bytes()?;
        for question in self.questions {
            bytes.append(&mut question.to_bytes());
        }
        for answer in self.answers {
            bytes.append(&mut answer.to_bytes()?);
        }
        for authority in self.authorities {
            bytes.append(&mut authority.to_bytes()?);
        }
        for additional in self.additionals {
            bytes.append(&mut additional.to_bytes()?)
        }
        Ok(bytes)
    }

    pub fn get_answer(&self) -> Option<&DnsRecord> {
        for answer in &self.answers {
            match answer.rdata {
                Rdata::A(_) | Rdata::CNAME(_) | Rdata::AAAA(_) | Rdata::MX(_) => {
                    return Some(answer);
                }
                _ => continue,
            }
        }
        None
    }

    pub fn get_nameserver_ip(&self) -> Option<&str> {
        for record in &self.additionals {
            if let Rdata::A(string) = &record.rdata {
                return Some(string);
            }
        }
        None
    }

    pub fn get_nameserver(&self) -> Option<&str> {
        for auth in &self.authorities {
            if let Rdata::NS(string) = &auth.rdata {
                return Some(string);
            }
        }
        None
    }

    pub fn packet_from_question(question: DnsQuestion) -> DnsPacket {
        let header = DnsHeader::simple_query_header();
        DnsPacket {
            header,
            questions: vec![question],
            answers: vec![],
            authorities: vec![],
            additionals: vec![],
        }
    }

    pub fn build_query(question: &DnsQuestion) -> Result<Vec<u8>, DnsError> {
        let header = DnsHeader::simple_query_header();
        let mut query_bytes = header.to_bytes()?;
        query_bytes.append(&mut question.to_bytes());
        Ok(query_bytes)
    }

    pub fn send_query(nameserver: &str, question: &DnsQuestion) -> Result<DnsPacket, DnsError> {
        // TODO different buf size?
        let mut buf: [u8; 1024] = [0; 1024];
        let query = Self::build_query(question)?;
        let socket = UdpSocket::bind("0.0.0.0:0")
            .map_err(|_| DnsError::NetworkError("Failed binding to socket"))?;
        let _res = socket
            .send_to(&query, (nameserver, 53))
            .map_err(|_| DnsError::NetworkError("Failed sending query"))?;
        let (_num_bytes, _src_addr) = socket
            .recv_from(&mut buf)
            .map_err(|_| DnsError::NetworkError("Failed receiving from socket"))?;
        DnsPacket::from_bytes(&buf)
    }

    pub fn make_error_response(self, err: DnsError) -> DnsPacket {
        let error_rcode = match err {
            DnsError::NotImplementedError(_) => HeaderFlags::RCODE_NOT_IMPL,
            DnsError::CacheError(_)
            | DnsError::ResolveError(_)
            | DnsError::EncodeError(_)
            | DnsError::NetworkError(_)
            | DnsError::DecodeError(_) => HeaderFlags::RCODE_SERVER_ERR,
        };
        let mut header = self.header;
        header.flags |= HeaderFlags::QR_RESPONSE as u16;
        header.flags |= error_rcode as u16;
        DnsPacket {
            header,
            questions: self.questions,
            answers: vec![],
            authorities: vec![],
            additionals: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rr_fields::Class;
    use crate::rr_fields::Type;
    use crate::util::encode_dns_name;
    use hex;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_dns_response_type_a() {
        let packet = "528a818000010001000000000a636f6d706c6574696f6e06616d617a6f6e03636f\
        6d0000010001c00c000100010000002500042cd78e8b";
        let packet_bytes = hex::decode(packet).unwrap();
        let expected = DnsPacket {
            header: DnsHeader {
                id: 0x528a,
                flags: 0x8180,
                num_questions: 1,
                num_answers: 1,
                num_authorities: 0,
                num_additionals: 0,
            },
            authorities: vec![],
            additionals: vec![],
            questions: vec![DnsQuestion {
                name: "completion.amazon.com".to_string(),
                qtype: Type::A,
                class: Class::CLASS_IN,
            }],
            answers: vec![DnsRecord {
                name: "completion.amazon.com".to_string(),
                class: Class::CLASS_IN,
                ttl: 37,
                rdata: Rdata::A("44.215.142.139".to_string()),
            }],
        };
        let decoded = DnsPacket::from_bytes(&packet_bytes);
        assert_eq!(decoded, Ok(expected));
    }
    #[test]
    fn test_dns_to_bytes_a() {
        let expected_str = "528a818000010001000000000a636f6d706c6574696f6e06616d617a6f6e03636f\
        6d00000100010a636f6d706c6574696f6e06616d617a6f6e03636f6d00000100010000002500042cd78e8b";
        let expected = hex::decode(expected_str).unwrap();
        let packet = DnsPacket {
            header: DnsHeader {
                id: 0x528a,
                flags: 0x8180,
                num_questions: 1,
                num_answers: 1,
                num_authorities: 0,
                num_additionals: 0,
            },
            authorities: vec![],
            additionals: vec![],
            questions: vec![DnsQuestion {
                name: "completion.amazon.com".to_string(),
                qtype: Type::A,
                class: Class::CLASS_IN,
            }],
            answers: vec![DnsRecord {
                name: "completion.amazon.com".to_string(),
                class: Class::CLASS_IN,
                ttl: 37,
                rdata: Rdata::A("44.215.142.139".to_string()),
            }],
        };
        let result: Vec<u8> = packet.to_bytes().unwrap();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_get_answer() {
        let record = DnsRecord {
            name: "encrypted-tbn0.gstatic.com".to_string(),
            class: Class::CLASS_IN,
            ttl: 96,
            rdata: Rdata::A("142.251.40.174".to_string()),
        };
        let packet = DnsPacket {
            header: DnsHeader {
                id: 0,
                flags: 0,
                num_questions: 0,
                num_answers: 0,
                num_authorities: 0,
                num_additionals: 0,
            },
            authorities: vec![],
            additionals: vec![],
            questions: vec![],
            answers: vec![record.clone()],
        };

        let result = packet.get_answer();
        let expected = Some(&record);
        assert_eq!(result, expected);
    }
    #[test]
    fn query_example() {
        let expected =
            String::from("3c5f0000000100000000000003777777076578616d706c6503636f6d0000010001");
        let question = DnsQuestion {
            name: "www.example.com".to_string(),
            qtype: Type::A,
            class: Class::CLASS_IN,
        };
        let res = DnsPacket::build_query(&question).unwrap();
        let res_hex = hex::encode(res);
        assert_eq!(res_hex[4..], expected[4..]);
    }
    #[test]
    fn test_encode_dns_name() {
        let expected = String::from("03777777076578616d706c6503636f6d00");
        let res = encode_dns_name("www.example.com");
        let res_hex = hex::encode(res);
        assert_eq!(res_hex, expected);
    }
}
