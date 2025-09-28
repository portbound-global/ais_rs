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
    Position(PositionClassA),
    Static(StaticData),
    NotN(NoTN),
    Unknown(u8), // for unsupported or malformed
}
#[derive(Debug)]
pub enum AISPartial {
    Position(PartialPositionClassA),
    Static(PartialStaticData),
    NotN(NoTN),
    Unknown(u8), // for unsupported or malformed
}

#[derive(Debug)]
pub struct PartialPositionClassA {
    pub mmsi: u32,
}

#[derive(Debug)]
pub struct PartialStaticData {
    pub mmsi: u32,
    pub imo: u32,
    pub ship_type: u8,
}

#[derive(Debug)]
pub struct PositionClassA {
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
    pub true_heading: u16,
    pub time_stamp: u8,
    pub maneuver_indicator: u8,
}

#[derive(Debug)]
pub struct StaticData {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub imo: u32,
    pub call_sign: String,
    pub vessel_name: String,
    pub ship_type: u8,
    pub dimension_to_bow: u16,
    pub dimension_to_stern: u16,
    pub dimension_to_port: u16,
    pub dimension_to_starboard: u16,
    pub draught: f32,
    pub destination: String,
}

#[derive(Debug)]
pub struct NoTN {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub aid_type: u8,
    pub aid_name: String,
    pub position_accuracy: bool,
    pub longitude: f32,
    pub latitude: f32,
    pub dimension_to_bow: u16,
    pub dimension_to_stern: u16,
    pub dimension_to_port: u16,
    pub dimension_to_starboard: u16,
    pub epfd: u8,
    pub time_stamp: u8,
    pub off_position: bool,
    pub regional_reserved: u16,
    pub raim_flag: bool,
    pub virtual_aid: bool,
    pub assigned_mode: bool,
    pub spare: bool,
    pub name_extension: String,
    
}