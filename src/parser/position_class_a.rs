use crate::parser::bit_parser::{get, BitField};
use crate::models::PositionClassA;

pub fn position_class_a(bits: Vec<u8>, message_type: u8) -> PositionClassA {
    PositionClassA {
        message_type,
        repeat_indicator: get(&bits, BitField { start: 6, len: 2 }),
        mmsi: get(&bits, BitField { start: 8, len: 30 }),
        navigation_status: get(&bits, BitField { start: 38, len: 4 }),
        rate_of_turn: get(&bits, BitField { start: 42, len: 8 }),
        speed_over_ground: get::<f32>(&bits, BitField { start: 50, len: 10 }) / 10.0,
        position_accuracy: get(&bits, BitField { start: 60, len: 1 }),
        longitude: get::<f32>(&bits, BitField { start: 61, len: 28 }) / 600000.0,
        latitude: get::<f32>(&bits, BitField { start: 89, len: 27 }) / 600000.0,
        course_over_ground: get::<f32>(&bits, BitField { start: 116, len: 12 }) / 10.0,
        true_heading: get(&bits, BitField { start: 128, len: 9 }),
        time_stamp: get(&bits, BitField { start: 137, len: 6 }),
        maneuver_indicator: get(&bits, BitField { start: 143, len: 2 }),
    }
}
