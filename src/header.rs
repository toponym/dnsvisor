use crate::cursor_read_num;
use crate::error::DnsError;
use rand::random;
use std::io::Cursor;
use std::io::Read;
#[derive(Debug, PartialEq, Clone)]
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
        let mut bytes = vec![];
        bytes.extend(self.id.to_be_bytes());
        bytes.extend(self.flags.to_be_bytes());
        bytes.extend(self.num_questions.to_be_bytes());
        bytes.extend(self.num_answers.to_be_bytes());
        bytes.extend(self.num_authorities.to_be_bytes());
        bytes.extend(self.num_additionals.to_be_bytes());
        Ok(bytes)
    }

    pub fn from_bytes(reader: &mut Cursor<&[u8]>) -> Result<Self, DnsError> {
        let mut buf_16 = [0u8; 2];
        Ok(Self {
            id: cursor_read_num!(reader, buf_16, u16::from_be_bytes),
            flags: cursor_read_num!(reader, buf_16, u16::from_be_bytes),
            num_questions: cursor_read_num!(reader, buf_16, u16::from_be_bytes),
            num_answers: cursor_read_num!(reader, buf_16, u16::from_be_bytes),
            num_authorities: cursor_read_num!(reader, buf_16, u16::from_be_bytes),
            num_additionals: cursor_read_num!(reader, buf_16, u16::from_be_bytes),
        })
    }

    pub fn simple_query_header() -> Self {
        let id: u16 = random();
        let no_recursion = 0;
        Self {
            id,
            flags: no_recursion,
            num_questions: 1,
            num_answers: 0,
            num_authorities: 0,
            num_additionals: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;
    use pretty_assertions::assert_eq;
    use std::io::Cursor;
    #[test]
    fn header_from_bytes_sample() {
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
    #[test]
    fn header_to_bytes_sample() {
        let header = DnsHeader {
            id: 24662,
            flags: 33152,
            num_questions: 1,
            num_answers: 1,
            num_authorities: 0,
            num_additionals: 0,
        };
        let expected_str = "605681800001000100000000";
        let expected = hex::decode(expected_str).unwrap();
        let result = header.to_bytes().unwrap();
        assert_eq!(expected, result);
    }
}
