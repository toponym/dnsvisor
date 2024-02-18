use crate::cache::DnsCache;
use crate::error::DnsError;
use crate::header::DnsHeader;
use crate::packet::DnsPacket;
use crate::question::DnsQuestion;
use crate::record::{DnsRecord, Rdata};
use crate::rr_fields::{Class, HeaderFlags, Type};
use async_recursion::async_recursion;
use log::{debug, info};
use std::collections::HashSet;
use std::sync::{Arc, Mutex, RwLock};

pub struct Resolver {
    pub cache: Arc<Mutex<DnsCache>>,
    pub blocklist: Arc<RwLock<HashSet<String>>>,
}

impl Resolver {
    pub fn new(blocklist: HashSet<String>) -> Self {
        Resolver {
            cache: Arc::new(Mutex::new(DnsCache::new())),
            blocklist: Arc::new(RwLock::new(blocklist)),
        }
    }

    fn build_response(
        mut header: DnsHeader,
        question: &DnsQuestion,
        answers: Vec<DnsRecord>,
    ) -> Result<DnsPacket, DnsError> {
        let num_answers = u16::try_from(answers.len()).map_err(|_| {
            DnsError::ResolveError("Number of answers exceeds u16 limit".to_string())
        })?;
        header.flags |= HeaderFlags::QR_RESPONSE as u16;
        let response_header = DnsHeader {
            id: header.id,
            flags: header.flags, // TODO custom flags?
            num_questions: 1,
            num_answers,
            num_authorities: 0,
            num_additionals: 0,
        };
        Ok(DnsPacket {
            header: response_header,
            questions: vec![question.clone()],
            answers,
            authorities: vec![],
            additionals: vec![],
        })
    }

    #[async_recursion]
    pub async fn resolve_packet(&mut self, query_packet: DnsPacket) -> Result<DnsPacket, DnsError> {
        // Verisign root nameserver
        let root_nameserver = String::from("198.41.0.4");
        let mut nameserver = root_nameserver;
        // Assuming there is only 1 question as RFC 1035 says this is typical.
        let orig_question = query_packet.questions.first().ok_or_else(|| {
            DnsError::ResolveError("Invalid request: no question supplied".to_string())
        })?;
        let mut domain_name = orig_question.name.clone();
        let record_type = orig_question.qtype;
        let mut answers: Vec<DnsRecord> = vec![];
        if let Ok(blocklist_lock) = self.blocklist.read() {
            if blocklist_lock.contains(&domain_name) {
                debug!("Blocklisted domain: {}", domain_name);
                let loopback_record = DnsRecord {
                    name: domain_name,
                    class: Class::CLASS_IN,
                    ttl: 43200,
                    rdata: Rdata::A("0.0.0.0".to_string()),
                };
                answers.push(loopback_record);
                let response = Self::build_response(query_packet.header, orig_question, answers);
                return response;
            }
        } else {
            return Err(DnsError::ResolveError(
                "Failed to unlock blocklist".to_string(),
            ));
        }
        loop {
            info!("Querying {} for {}", nameserver, domain_name);
            let question = DnsQuestion::new(&domain_name, record_type, Class::CLASS_IN);
            // check cache
            if let Ok(mut cache_lock) = self.cache.lock() {
                if let Some(record) = cache_lock.lookup(&question) {
                    debug!("Cache hit");
                    answers.push(record.clone());
                    let response =
                        Self::build_response(query_packet.header, orig_question, answers);
                    return response;
                }
            } else {
                return Err(DnsError::ResolveError("Failed to unlock cache".to_string()));
            }
            debug!("Cache miss");
            // otherwise ask remote resolver
            let response = DnsPacket::send_query(&nameserver, &question)?;
            if let Ok(mut cache_lock) = self.cache.lock() {
                cache_lock.cache_answers(&response)?;
            } else {
                return Err(DnsError::ResolveError("Failed to unlock cache".to_string()));
            }
            if let Some(answer) = response.get_answer() {
                match &answer.rdata {
                    Rdata::A(string) => {
                        let answer_string = string;
                        debug!("Got ip: {}", answer_string);
                        answers.push(answer.clone());
                        let response =
                            Self::build_response(query_packet.header, orig_question, answers);
                        return response;
                    }
                    Rdata::CNAME(string) => {
                        let answer_string = string;
                        debug!("Got CNAME domain: {}", answer_string);
                        answers.push(answer.clone());
                        domain_name = answer_string.clone();
                    }
                    Rdata::MX(rdata_mx) => {
                        debug!("Got MX record: {}", rdata_mx.exchange);
                        answers.push(answer.clone());
                        let response =
                            Self::build_response(query_packet.header, orig_question, answers);
                        return response;
                    }
                    _ => {
                        return Err(DnsError::ResolveError(format!(
                            "Unexpected answer type: {:?}",
                            answer.get_type()
                        )))
                    }
                }
            } else if let Some(ns_ip) = response.get_nameserver_ip() {
                debug!("Got nameserver ip: {}", ns_ip);
                nameserver = ns_ip.to_string();
            } else if let Some(ns_domain) = response.get_nameserver() {
                debug!("Got nameserver domain: {}", ns_domain);
                nameserver = self.resolve(ns_domain, Type::A).await?; // TODO is Type A right?
            } else {
                return Err(DnsError::ResolveError(format!(
                    "Unexpected response: {:?}",
                    response
                )));
            }
        }
    }

    #[async_recursion]
    pub async fn resolve(
        &mut self,
        req_domain_name: &str,
        record_type: Type,
    ) -> Result<String, DnsError> {
        // Verisign root nameserver
        let question = DnsQuestion::new(req_domain_name, record_type, Class::CLASS_IN);
        let query_packet = DnsPacket::packet_from_question(question);
        let response_packet = self.resolve_packet(query_packet).await?;
        for answer in response_packet.answers {
            if answer.get_type() == record_type {
                match answer.rdata {
                    Rdata::A(string) => return Ok(string),
                    Rdata::AAAA(string) => return Ok(string),
                    Rdata::CNAME(string) => return Ok(string),
                    _ => continue,
                }
            }
        }
        Err(DnsError::ResolveError(format!(
            "No valid answer returned for requested type: {:?}",
            record_type
        )))
    }
}
impl Default for Resolver {
    fn default() -> Self {
        Self::new(HashSet::new())
    }
}
