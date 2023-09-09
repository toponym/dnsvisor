use serde::{Deserialize, Serialize};
use std::io::Cursor;
#[derive(Serialize, Deserialize, Debug)]
pub struct DnsQuestion {
    pub name: Vec<u8>,
    pub qtype: u16,
    pub class: u16,
}

impl DnsQuestion {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.name.clone();
        bytes.extend_from_slice(&self.qtype.to_be_bytes());
        bytes.extend_from_slice(&self.class.to_be_bytes());
        bytes
    }

    pub fn from_bytes(reader: &mut Cursor<&[u8]>) -> DnsQuestion {
        unimplemented!();
    }
}
