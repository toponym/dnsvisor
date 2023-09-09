use crate::header::DnsHeader;
use crate::question::DnsQuestion;
use crate::record::DnsRecord;
use std::io::Cursor;
#[derive(Debug)]
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
            parse_num_items!(&mut reader, header.num_questions, DnsRecord::from_bytes);
        let authorities: Vec<DnsRecord> =
            parse_num_items!(&mut reader, header.num_questions, DnsRecord::from_bytes);
        let additionals: Vec<DnsRecord> =
            parse_num_items!(&mut reader, header.num_questions, DnsRecord::from_bytes);
        Self {
            header,
            questions,
            answers,
            authorities,
            additionals,
        }
    }
}
