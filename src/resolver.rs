use crate::cache::DnsCache;
use crate::error::DnsError;
use crate::header::DnsHeader;
use crate::packet::DnsPacket;
use crate::question::DnsQuestion;
use crate::record::DnsRecord;
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

    fn build_response(header: &DnsHeader, question: &DnsQuestion, record: &DnsRecord) -> DnsPacket {
        let response_header = DnsHeader {
            id: header.id,
            flags: header.flags, // TODO custom flags?
            num_questions: 1,
            num_answers: 1,
            num_authorities: 0,
            num_additionals: 0,
        };
        DnsPacket {
            header: response_header,
            questions: vec![question.clone()],
            answers: vec![record.clone()],
            authorities: vec![],
            additionals: vec![],
        }
    }

    pub fn resolve_packet(&mut self, query_packet: DnsPacket) -> Result<DnsPacket, DnsError> {
        // Verisign root nameserver
        let root_nameserver = String::from("198.41.0.4");
        let mut nameserver = root_nameserver;
        // Assuming there is only 1 question as RFC 1035 says this is typical.
        let question = query_packet.questions.get(0).ok_or_else(|| {
            DnsError::ResolveError("Invalid request: no question supplied".to_string())
        })?;
        let header = &query_packet.header;
        let mut domain_name = question.name.clone();
        let record_type = question.qtype;
        loop {
            info!("Querying {} for {}", nameserver, domain_name);
            let question = DnsQuestion::new(&domain_name, record_type, Class::CLASS_IN);
            // check cache
            if let Some(record) = self.cache.lookup(&question) {
                debug!("Cache hit");
                let response = Self::build_response(header, &question, record);
                return Ok(response);
            }
            debug!("Cache miss");
            // otherwise ask remote resolver
            let response = DnsPacket::send_query(&nameserver, &question)?;
            self.cache.cache_answers(&response)?;
            if let Some(answer) = response.get_answer() {
                match answer.rtype {
                    Type::A => {
                        let answer_string = answer.data.to_string();
                        debug!("Got ip: {}", answer_string);
                        let response = Self::build_response(header, &question, answer);
                        return Ok(response);
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
            let response = DnsPacket::send_query(&nameserver, &question)?;
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
