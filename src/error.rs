#[derive(Debug, PartialEq, Eq)]
pub enum DnsError {
    ResolveError(String),
    EncodeError(&'static str),
    DecodeError(&'static str),
    NetworkError(&'static str),
    CacheError(&'static str),
}
