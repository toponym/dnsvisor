# dnsvisor
A DNS Server in Rust

# Usage
`dnsvisor` can be used as a server, interactively, or as a library.

Features include:
- Supported records: `A`, `AAAA`, `MX`, `CNAME`, `SOA`, `NS`
- Caching
- Domain blocking like pihole

## Server
There is a server mode which handles DNS requests. It has been tested with `dig`, but not as the dedicated DNS server for a system.
### Running the Server
Start the server with:
`cargo run server 127.0.0.1 1053`
and then send a query with:
`dig +noedns @127.0.0.1 -p 1053 example.com`
This server doesn't support extended DNS, so `+noedns` is important.
### Server blocklist
Specify a blocklist with `cargo run server 127.0.0.1 1053 -b blocklist.txt`
The blocklist format is:
```
# comment
full-domain-name.net
another-domain-name.com
```
A resource for blocklists is: [dns-blocklists](https://github.com/hagezi/dns-blocklists).
### Browser script
The script `browse.zsh` will use `dig` to query this server and open the webpage in your browser. Helpful for confirming the retrieved IPs are correct.

## Interactive
Running `cargo run interactive` opens a prompt to query domain names.
```
Enter a domain> target.com
Domain IP: 151.101.2.187
Enter a domain> news.ycombinator.com 
Domain IP: 209.216.230.207
```

## Library
See `examples/basic-resolver.rs` for an example. You can run it with `cargo run --example resolver facebook.com`

# Testing and debugging
Enable debug logging with `export RUST_LOG=debug`
Run tests with `cargo test`
## Coverage
This project uses [grcov](https://github.com/mozilla/grcov) to track code coverage.
After installing `grcov`, generate a coverage report with `make coverage`.
Open `./target/debug/coverage/index.html` to see the coverage report.
