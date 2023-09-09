use crate::header::DnsHeader;
use crate::question::DnsQuestion;
use crate::record::DnsRecord;
use std::io::Cursor;
pub struct DnsPacket {
    header: Vec<DnsHeader>,
}

impl DnsPacket {
    pub fn from_bytes(buf: &[u8]) -> Self {
        let mut reader = Cursor::new(buf);
        let header = DnsHeader::from_bytes(&mut reader);
        let questions: Vec<DnsQuestion> = (0..header.num_questions)
            .map(|_| DnsQuestion::from_bytes(&mut reader))
            .collect();
        let answers: Vec<DnsRecord> = (0..header.num_answers)
            .map(|_| DnsRecord::from_bytes(&mut reader))
            .collect();
        unimplemented!();
    }
}
