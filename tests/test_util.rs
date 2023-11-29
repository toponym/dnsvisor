#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    #[should_panic(expected = "Malformed DNS record: pointer loop detected")]
    fn decode_detect_loop() {
        // input has two compressed dns pointers which create a loop
        let input = vec![0b1100_0010, 0b0000_0000, 0b1100_0000, 0b0000_0000];
        let mut reader = Cursor::new(input.as_slice());
        dnsvisor::util::decode_dns_name(&mut reader);
    }
}