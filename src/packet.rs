use crate::error::DnsError;
use crate::header::DnsHeader;
use crate::question::DnsQuestion;
use crate::record::DnsRecord;
use crate::rr_fields::Type;
use std::io::Cursor;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct DnsPacket {
    header: DnsHeader,
    questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    authorities: Vec<DnsRecord>,
    additionals: Vec<DnsRecord>,
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

    pub fn get_answer(&self) -> Option<&DnsRecord> {
        for answer in &self.answers {
            let answer_type = answer.rtype;
            if answer_type == Type::A || answer_type == Type::CNAME {
                return Some(answer);
            }
        }
        None
    }

    pub fn get_nameserver_ip(&self) -> Option<&str> {
        for record in &self.additionals {
            if record.rtype == Type::A {
                return Some(&record.data);
            }
        }
        None
    }

    pub fn get_nameserver(&self) -> Option<&str> {
        for auth in &self.authorities {
            if auth.rtype == Type::NS {
                return Some(&auth.data);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rr_fields::Class;
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
                rtype: Type::A,
                class: Class::CLASS_IN,
                ttl: 37,
                data: "44.215.142.139".to_string(),
            }],
        };
        let decoded = DnsPacket::from_bytes(&packet_bytes);
        assert_eq!(decoded, Ok(expected));
    }
    #[test]
    fn test_get_answer() {
        let record = DnsRecord {
            name: "encrypted-tbn0.gstatic.com".to_string(),
            rtype: Type::A,
            class: Class::CLASS_IN,
            ttl: 96,
            data: "142.251.40.174".to_string(),
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
}
