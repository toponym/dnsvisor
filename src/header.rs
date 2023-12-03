use crate::error::DnsError;
use bincode::Options;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DnsHeader {
    pub id: u16,
    pub flags: u16,
    pub num_questions: u16,
    pub num_answers: u16,
    pub num_authorities: u16,
    pub num_additionals: u16,
}

impl DnsHeader {
    pub fn to_bytes(&self) -> Result<Vec<u8>, DnsError> {
        let options = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding();
        options
            .serialize(self)
            .map_err(|_| DnsError::EncodeError("Failed to encode DNS header"))
    }

    pub fn from_bytes(reader: &mut Cursor<&[u8]>) -> Result<Self, DnsError> {
        let mut buf: [u8; 12] = [0; 12];
        reader
            .read_exact(&mut buf)
            .map_err(|_| DnsError::DecodeError("Failed to decode DNS header"))?;
        let options = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding();
        options
            .deserialize(&buf)
            .map_err(|_| DnsError::DecodeError("Failed to decode DNS header"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;
    use pretty_assertions::assert_eq;
    use std::io::Cursor;
    #[test]
    fn header_sample_response() {
        let response = "60568180000100010000000003777777076578616d706c6503636f6d0000010001c00c000100010000529b00045db8d822";
        let response_bytes = hex::decode(response).unwrap();
        let mut reader = Cursor::new(response_bytes.as_slice());
        let header = DnsHeader::from_bytes(&mut reader);
        let expected = DnsHeader {
            id: 24662,
            flags: 33152,
            num_questions: 1,
            num_answers: 1,
            num_authorities: 0,
            num_additionals: 0,
        };
        let expected_end_pos = 12;
        assert_eq!(header, Ok(expected));
        assert_eq!(expected_end_pos, reader.position());
    }
}
