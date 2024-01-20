#[derive(Debug, PartialEq, Eq)]
pub enum DnsError {
    ResolveError(String),
    EncodeError(&'static str),
    DecodeError(String),
    NetworkError(&'static str),
    CacheError(&'static str),
    NotImplementedError(String),
}
