use crate::record::DnsRecord;
use crate::question::DnsQuestion;
use std::time::Instant;

pub struct DnsCache {
    cache: HashMap<String, DnsCacheEntry>, // TODO best data structure for storing cache entries?
}

impl DnsCache {
    fn resolve(question: DnsQuestion) -> Option<DnsRecord> {
        unimplemented!();
    }
}


struct DnsCacheEntry {
    record: DnsRecord,
    expires: Instant,
}

impl DnsCacheEntry {
    fn expired(self) -> bool {
        unimplemented!();
    }
}
