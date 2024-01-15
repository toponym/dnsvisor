use crate::cache::DnsCache;
use crate::error::DnsError;
use crate::query;
use crate::question::DnsQuestion;
use crate::rr_fields::{Class, Type};
use log::{debug, info};

pub struct Resolver {
    cache: DnsCache,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            cache: DnsCache::new(),
        }
    }

    pub fn resolve(
        &mut self,
        req_domain_name: &str,
        record_type: Type,
    ) -> Result<String, DnsError> {
        // Verisign root nameserver
        let root_nameserver = String::from("198.41.0.4");
        let mut nameserver = root_nameserver;
        let mut domain_name = String::from(req_domain_name);
        loop {
            info!("Querying {} for {}", nameserver, domain_name);
            let question = DnsQuestion::new(&domain_name, record_type, Class::CLASS_IN);
            // check cache
            if let Some(record) = self.cache.lookup(&question) {
                debug!("Cache hit");
                return Ok(record.data.clone());
            }
            debug!("Cache miss");
            // otherwise ask remote resolver
            let response = query::send_query(&nameserver, &question)?;
            self.cache.cache_answers(&response)?;
            if let Some(answer) = response.get_answer() {
                match answer.rtype {
                    Type::A => {
                        let answer_string = answer.data.to_string();
                        debug!("Got ip: {}", answer_string);
                        return Ok(answer_string);
                    }
                    Type::CNAME => {
                        let answer_string = answer.data.to_string();
                        debug!("Got CNAME domain: {}", answer_string);
                        domain_name = answer_string;
                    }
                    _ => {
                        return Err(DnsError::ResolveError(format!(
                            "Unexpected answer type: {:?}",
                            answer.rtype
                        )))
                    }
                }
            } else if let Some(ns_ip) = response.get_nameserver_ip() {
                debug!("Got nameserver ip: {}", ns_ip);
                nameserver = ns_ip.to_string();
            } else if let Some(ns_domain) = response.get_nameserver() {
                debug!("Got nameserver domain: {}", ns_domain);
                nameserver = self.resolve(ns_domain, Type::A)?; // TODO is Type A right?
            } else {
                return Err(DnsError::ResolveError(format!(
                    "Unexpected response: {:?}",
                    response
                )));
            }
        }
    }
}
impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}
