use std::io::Cursor;
pub struct DnsRecord {
    name: [u8; 253],
    rtype: u32,
    class: u32,
    tt1: u32,
    data: Vec<u8>,
}

impl DnsRecord {
    pub fn from_bytes(reader: &mut Cursor<&[u8]>) -> Self {
        unimplemented!();
    }
}
