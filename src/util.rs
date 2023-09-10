use std::io::Cursor;
use std::io::Read;

pub fn encode_dns_name(domain_name: &str) -> Vec<u8> {
    assert!(domain_name.is_ascii());
    let mut encoded: Vec<u8> = vec![];
    for part in domain_name.split('.') {
        assert!(part.len() <= 63);
        let length = part.len() as u8;
        encoded.push(length);
        encoded.extend_from_slice(part.as_bytes());
    }
    encoded.push(0);
    encoded
}

pub fn decode_dns_name(reader: &mut Cursor<&[u8]>) -> String {
    let mut parts: Vec<String> = vec![];
    let mut part: [u8; 63] = [0; 63];
    let mut length_buf: [u8; 1] = [0; 1];
    reader.read_exact(&mut length_buf).unwrap();
    while length_buf[0] != 0 {
        let length = length_buf[0] as u64;
        if length_buf[0] == 0b1100_0000 {
            parts.push(decode_compressed_name(length, reader));
            break;
        } else {
            let _ = reader.take(length).read(&mut part).unwrap();
            parts.push(String::from_utf8((part[0..(length as usize)]).to_vec()).unwrap());
            part.iter_mut().for_each(|x| *x = 0);
        }
        reader.read_exact(&mut length_buf).unwrap();
    }
    parts.join(".")
}

fn decode_compressed_name(length: u64, reader: &mut Cursor<&[u8]>) -> String {
    let mut byte: [u8; 1] = [0; 1];
    reader.read_exact(&mut byte).unwrap();
    let pointer: u64 = (length & 0b0011_1111) + byte[0] as u64;
    let prev_position = reader.position();
    reader.set_position(pointer);
    let res = decode_dns_name(reader);
    reader.set_position(prev_position);
    res
}
