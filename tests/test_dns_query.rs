#[cfg(test)]
mod tests {
    use dnsvisor::query::{build_query, encode_dns_name};
    use dnsvisor::rr_fields::Type;
    use hex;
    use pretty_assertions::assert_eq;

    #[test]
    fn query_example() {
        let expected =
            String::from("3c5f0100000100000000000003777777076578616d706c6503636f6d0000010001");
        let res = build_query(String::from("www.example.com"), Type::A);
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
