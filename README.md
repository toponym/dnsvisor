# dnsvisor
A DNS Server in Rust


# Usage
Run the resolver which uses the dnsvisor library with: `cargo run --example resolver facebook.com`

Enable debug logging with `export RUST_LOG=debug`

# Testing
Run tests with `cargo test`

This project uses [grcov](https://github.com/mozilla/grcov) to track code coverage.
After installing `grcov`, generate a coverage report with `make coverage`.
Open `./target/debug/coverage/index.html` to see the coverage report.

# Todo
- Make a server mode of the resolver which handles valid DNS requests and returns valid DNS responses
- DNS blocking like pihole
