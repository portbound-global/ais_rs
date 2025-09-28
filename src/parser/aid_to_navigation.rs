use crate::parser::bit_parser::{get, BitField};
use crate::models::NoTN;

pub fn aid_to_navigation(bits: Vec<u8>, message_type: u8) -> NoTN {
    NoTN {
        message_type,
        repeat_indicator: get(&bits, BitField { start: 6, len: 2 }),
        mmsi: get(&bits, BitField { start: 8, len: 30 }),
        aid_type: get(&bits, BitField { start: 38, len: 5 }),
        aid_name: get(&bits, BitField { start: 43, len: 120 }),
        position_accuracy: get(&bits, BitField { start: 163, len: 1 }),
        longitude: get::<f32>(&bits, BitField { start: 164, len: 28 }) / 600000.0,
        latitude: get::<f32>(&bits, BitField { start: 192, len: 27 }) / 600000.0,
        dimension_to_bow: get(&bits, BitField { start: 219, len: 9 }),
        dimension_to_stern: get(&bits, BitField { start: 228, len: 9 }),
        dimension_to_port: get(&bits, BitField { start: 237, len: 6 }),
        dimension_to_starboard: get(&bits, BitField { start: 243, len: 6 }),
        epfd: get(&bits, BitField { start: 249, len: 4 }),
        time_stamp: get(&bits, BitField { start: 253, len: 6 }),
        off_position: get(&bits, BitField { start: 259, len: 1 }),
        regional_reserved: get(&bits, BitField { start: 260, len: 8 }),
        raim_flag: get(&bits, BitField { start: 268, len: 1 }),
        virtual_aid: get(&bits, BitField { start: 269, len: 1 }),
        assigned_mode: get(&bits, BitField { start: 270, len: 1 }),
        spare: get(&bits, BitField { start: 271, len: 1 }),
        name_extension: String::new(),
    }
}
