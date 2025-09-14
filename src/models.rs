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

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub enum AISMessage {
    Position(AISPositionClassA),
    Static(AISStaticData),
    Unknown(u8), // for unsupported or malformed
}

#[derive(Debug)]
pub struct AISPositionClassA {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub navigation_status: u8,
    pub rate_of_turn: i8,
    pub speed_over_ground: f32,
    pub position_accuracy: bool,
    pub longitude: f32,
    pub latitude: f32,
    pub course_over_ground: f32,
    pub true_heading: usize,
    pub time_stamp: usize,
    pub maneuver_indicator: u8,
}

#[derive(Debug)]
pub struct AISStaticData {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub imo: u32,
    pub call_sign: String,
    pub vessel_name: String,
    pub ship_type: usize,
    pub dimension_to_bow: usize,
    pub dimension_to_stern: usize,
    pub dimension_to_port: usize,
    pub dimension_to_starboard: usize,

    pub draught: f32,
    pub destination: String,
}
