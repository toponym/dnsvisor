use crate::error::DnsError;
use crate::question::DnsQuestion;
use crate::record::DnsRecord;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct DnsCache {
    // The key of a DnsCache row is the fields of a DnsQuestion
    // TODO switch key to u64 hash? (check for performance difference)
    cache: HashMap<DnsQuestion, DnsCacheEntry>,
}

impl DnsCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn lookup(&mut self, question: DnsQuestion) -> Option<&DnsRecord> {
        // Delete cache entry if expired
        if let Some(entry) = self.cache.get(&question) {
            if entry.expired() {
                self.cache.remove(&question);
            }
        }
        self.cache.get(&question).map(|x| &x.record)
    }

    pub fn add(&mut self, record: DnsRecord) -> Result<(), DnsError> {
        let question = record.get_question();
        let entry = DnsCacheEntry::new(record)?;
        self.cache.insert(question, entry);
        Ok(())
    }
}

struct DnsCacheEntry {
    record: DnsRecord,
    expires: Instant,
}

impl DnsCacheEntry {
    pub fn new(record: DnsRecord) -> Result<Self, DnsError> {
        let now = Instant::now();
        let ttl_duration = Duration::from_secs(record.ttl as u64);
        if let Some(expires) = now.checked_add(ttl_duration) {
            Ok(Self { record, expires })
        } else {
            Err(DnsError::CacheError(
                "Failed to create expiration time for cache record",
            ))
        }
    }

    fn expired(&self) -> bool {
        let now = Instant::now();
        now >= self.expires
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rr_fields::*;
    use pretty_assertions::assert_eq;
    use std::thread;
    #[test]
    fn entry_not_expired() {
        let ttl = 5;
        let expected = false;
        let record = DnsRecord {
            name: String::from("placeholder"),
            rtype: Type::A,
            class: Class::CLASS_IN,
            ttl: ttl,
            data: String::from(""),
        };
        let entry = DnsCacheEntry::new(record).unwrap();
        let res = entry.expired();
        assert_eq!(expected, res);
    }
    #[test]
    fn entry_expired() {
        let ttl: u32 = 1;
        let sleep_time = Duration::from_secs((ttl + 1) as u64);
        let expected = true;
        let record = DnsRecord {
            name: String::from("example.com"),
            rtype: Type::A,
            class: Class::CLASS_IN,
            ttl: ttl,
            data: String::from(""),
        };
        let entry = DnsCacheEntry::new(record);
        thread::sleep(sleep_time);
        let res = entry.unwrap().expired();
        assert_eq!(expected, res);
    }

    #[test]
    fn cache_lookup_empty() {
        let mut cache = DnsCache {
            cache: HashMap::new(),
        };
        let query = DnsQuestion {
            name: "example.com".to_string(),
            qtype: Type::A,
            class: Class::CLASS_IN,
        };
        let expected = None;
        assert_eq!(cache.lookup(query), expected);
    }
    #[test]
    fn cache_lookup_hit() {
        let mut cache = DnsCache {
            cache: HashMap::new(),
        };
        let record = DnsRecord {
            name: "example.com".to_string(),
            rtype: Type::A,
            class: Class::CLASS_IN,
            ttl: 5,
            data: "127.0.0.1".to_string(),
        };
        let question = record.get_question();
        cache.add(record.clone()).unwrap();
        let expected = Some(&record);
        let result = cache.lookup(question);
        assert_eq!(result, expected);
    }
    #[test]
    fn cache_lookup_expired() {
        let mut cache = DnsCache {
            cache: HashMap::new(),
        };
        let record = DnsRecord {
            name: "example.com".to_string(),
            rtype: Type::A,
            class: Class::CLASS_IN,
            ttl: 0,
            data: "127.0.0.1".to_string(),
        };
        let question = record.get_question();
        cache.add(record).unwrap();
        let expected = None;
        let result = cache.lookup(question);
        assert_eq!(result, expected);
        assert!(cache.cache.is_empty())
    }
}
