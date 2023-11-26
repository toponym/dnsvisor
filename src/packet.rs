use crate::header::DnsHeader;
use crate::question::DnsQuestion;
use crate::record::DnsRecord;
use crate::rr_fields::Type;
use std::io::Cursor;

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsPacket {
    header: DnsHeader,
    questions: Vec<DnsQuestion>,
    answers: Vec<DnsRecord>,
    authorities: Vec<DnsRecord>,
    additionals: Vec<DnsRecord>,
}
macro_rules! parse_num_items {
    ($reader: expr, $num: expr, $parser: path) => {
        (0..$num).map(|_| $parser($reader)).collect()
    };
}

impl DnsPacket {
    pub fn from_bytes(buf: &[u8]) -> Self {
        let mut reader = Cursor::new(buf);
        let header = DnsHeader::from_bytes(&mut reader);
        let questions: Vec<DnsQuestion> =
            parse_num_items!(&mut reader, header.num_questions, DnsQuestion::from_bytes);
        let answers: Vec<DnsRecord> =
            parse_num_items!(&mut reader, header.num_answers, DnsRecord::from_bytes);
        let authorities: Vec<DnsRecord> =
            parse_num_items!(&mut reader, header.num_authorities, DnsRecord::from_bytes);
        let additionals: Vec<DnsRecord> =
            parse_num_items!(&mut reader, header.num_additionals, DnsRecord::from_bytes);
        Self {
            header,
            questions,
            answers,
            authorities,
            additionals,
        }
    }

    pub fn get_answer(&self) -> Option<&str> {
        for answer in &self.answers {
            if answer.rtype == Type::A {
                return Some(&answer.data);
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
