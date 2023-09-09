use bincode::Options;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct DnsHeader {
    pub id: u16,
    pub flags: u16,
    pub num_questions: u16,
    pub num_answers: u16,
    pub num_authorities: u16,
    pub num_additionals: u16,
}

impl DnsHeader {
    pub fn to_bytes(&self) -> Vec<u8> {
        let options = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding();
        options.serialize(self).unwrap()
    }

    pub fn from_bytes(reader: &mut Cursor<&[u8]>) -> Self {
        let mut buf: [u8; 12] = [0; 12];
        reader.read_exact(&mut buf).unwrap();
        let options = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding();
        options.deserialize(&buf).unwrap()
    }
}
