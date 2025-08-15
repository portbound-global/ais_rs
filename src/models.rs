use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug)]
pub struct NmeaSentence {
    pub talker: String,
    pub total_sentences: u8,
    pub sentence_number: u8,
    pub sentence_id: Option<u8>,
    pub channel: char,
    pub payload: String,
    pub fill_bits: u8,
}

#[derive(Debug)]
pub struct SentenceGroup {
    pub total_sentences: u8,
    pub payloads: HashMap<u8, String>,
    pub fill_bits: u8,
    pub created_at: Instant,
}
