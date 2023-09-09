use std::net::UdpSocket;

pub fn query_google(query: &[u8]) {
    let mut buf: [u8; 1024] = [0; 1024];
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    println!("Sending");
    let google_dns = "8.8.8.8:53";
    let res = socket.send_to(query, google_dns).unwrap();
    println!("Sent {} bytes", res);
    println!("Receiving");
    let (num_bytes, src_addr) = socket.recv_from(&mut buf).unwrap();
    println!("Received {} bytes from {}", num_bytes, src_addr);
    println!("Message: {:x?}", buf);
}
