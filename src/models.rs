use std::time::Instant;

pub struct NmeaSentence {
    pub talker: String,
    pub payload: String,
    pub channel: char,
    pub total_sentences: u8,
    pub sentence_number: u8,
    pub fill_bits: u8,
    pub sentence_id: Option<u8>,
}

#[derive(Debug)]
pub struct BuildSentence {
    pub payload: String,
    pub talker: String,
    pub channel: char,
    pub fill_bits: u8,
    pub timestamp: Instant,
}

#[derive(Debug)]
pub struct PartialSentence {
    pub payloads: Vec<String>,
    pub total_sentences: u8,
    pub fill_bits: u8,
    pub payload_len: u8,
    pub available: bool,
}
