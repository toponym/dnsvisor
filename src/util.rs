use crate::error::DnsError;
use std::collections::HashSet;
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

pub fn decode_dns_name(reader: &mut Cursor<&[u8]>) -> Result<String, DnsError> {
    inner_decode_dns_name(reader, &mut HashSet::new())
}
fn inner_decode_dns_name(
    reader: &mut Cursor<&[u8]>,
    visited: &mut HashSet<u64>,
) -> Result<String, DnsError> {
    let mut parts: Vec<String> = vec![];
    let mut part: [u8; 63] = [0; 63];
    let mut length_buf: [u8; 1] = [0; 1];
    reader
        .read_exact(&mut length_buf)
        .map_err(|_| DnsError::DecodeError("Failed decoding DNS name: while reading length"))?;
    while length_buf[0] != 0 {
        let length = length_buf[0] as u64;
        if (length_buf[0] & 0b1100_0000) != 0 {
            parts.push(decode_compressed_name(length, reader, visited)?);
            break;
        } else {
            let _ = reader.take(length).read(&mut part).map_err(|_| {
                DnsError::DecodeError("Failed decoding DNS name: while reading name segment")
            })?;
            parts.push(
                String::from_utf8((part[0..(length as usize)]).to_vec()).map_err(|_| {
                    DnsError::DecodeError("Failed decoding DNS name: while decoding name segment")
                })?,
            );
            part.iter_mut().for_each(|x| *x = 0);
        }
        reader
            .read_exact(&mut length_buf)
            .map_err(|_| DnsError::DecodeError("Failed decoding DNS name: while reading length"))?;
    }
    Ok(parts.join("."))
}

fn decode_compressed_name(
    length: u64,
    reader: &mut Cursor<&[u8]>,
    visited: &mut HashSet<u64>,
) -> Result<String, DnsError> {
    let mut byte: [u8; 1] = [0; 1];
    reader.read_exact(&mut byte).map_err(|_| {
        DnsError::DecodeError("Failed decoding compressed DNS name: while reading pointer byte")
    })?;
    let pointer: u64 = (length & 0b0011_1111) + byte[0] as u64;
    if visited.contains(&pointer) {
        return Err(DnsError::DecodeError(
            "Malformed DNS record: pointer loop detected",
        ));
    }
    visited.insert(pointer);
    let prev_position = reader.position();
    reader.set_position(pointer);
    let res = inner_decode_dns_name(reader, visited);
    reader.set_position(prev_position);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn decode_detect_loop() {
        // input has two compressed dns pointers which create a loop
        let input = vec![0b1100_0010, 0b0000_0000, 0b1100_0000, 0b0000_0000];
        let mut reader = Cursor::new(input.as_slice());
        let expected = Err(DnsError::DecodeError(
            "Malformed DNS record: pointer loop detected",
        ));
        let res = decode_dns_name(&mut reader);
        assert_eq!(expected, res)
    }
}
