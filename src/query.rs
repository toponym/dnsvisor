use crate::error::DnsError;
use crate::header::DnsHeader;
use crate::packet::DnsPacket;
use crate::question::DnsQuestion;

use rand::random;
use std::net::UdpSocket;

pub fn build_query(question: &DnsQuestion) -> Result<Vec<u8>, DnsError> {
    let id: u16 = random();
    let no_recursion = 0;
    let header = DnsHeader {
        id,
        flags: no_recursion,
        num_questions: 1,
        num_answers: 0,
        num_authorities: 0,
        num_additionals: 0,
    };
    let mut query_bytes = header.to_bytes()?;
    query_bytes.append(&mut question.to_bytes());
    Ok(query_bytes)
}

pub fn send_query(nameserver: &str, question: &DnsQuestion) -> Result<DnsPacket, DnsError> {
    // TODO different buf size?
    let mut buf: [u8; 1024] = [0; 1024];
    let query = build_query(question)?;
    let socket = UdpSocket::bind("0.0.0.0:0")
        .map_err(|_| DnsError::NetworkError("Failed binding to socket"))?;
    let _res = socket
        .send_to(&query, (nameserver, 53))
        .map_err(|_| DnsError::NetworkError("Failed sending query"))?;
    let (_num_bytes, _src_addr) = socket
        .recv_from(&mut buf)
        .map_err(|_| DnsError::NetworkError("Failed receiving from socket"))?;
    DnsPacket::from_bytes(&buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rr_fields::Class;
    use crate::rr_fields::Type;
    use crate::util::encode_dns_name;
    use hex;
    use pretty_assertions::assert_eq;

    #[test]
    fn query_example() {
        let expected =
            String::from("3c5f0000000100000000000003777777076578616d706c6503636f6d0000010001");
        let question = DnsQuestion {
            name: "www.example.com".to_string(),
            qtype: Type::A,
            class: Class::CLASS_IN,
        };
        let res = build_query(&question).unwrap();
        let res_hex = hex::encode(res);
        assert_eq!(res_hex[4..], expected[4..]);
    }
    #[test]
    fn test_encode_dns_name() {
        let expected = String::from("03777777076578616d706c6503636f6d00");
        let res = encode_dns_name("www.example.com");
        let res_hex = hex::encode(res);
        assert_eq!(res_hex, expected);
    }
}
