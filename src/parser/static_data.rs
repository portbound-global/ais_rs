use crate::parser::bit_parser::{get, BitField};
use crate::models::AISStaticData;

pub fn static_data(bits: Vec<u8>, message_type: u8) -> AISStaticData {
    AISStaticData {
        message_type,
        repeat_indicator: get(&bits, BitField { start: 6, len: 2 }),
        mmsi: get(&bits, BitField { start: 8, len: 30 }),
        imo: get(&bits, BitField { start: 40, len: 30 }),
        call_sign: get(&bits, BitField { start: 70, len: 42 }),
        vessel_name: get(
            &bits,
            BitField {
                start: 112,
                len: 120,
            },
        ),
        ship_type: get(&bits, BitField { start: 232, len: 8 }),
        dimension_to_bow: get(&bits, BitField { start: 240, len: 9 }),
        dimension_to_stern: get(&bits, BitField { start: 249, len: 9 }),
        dimension_to_port: get(&bits, BitField { start: 258, len: 6 }),
        dimension_to_starboard: get(&bits, BitField { start: 264, len: 6 }),

        draught: get::<f32>(&bits, BitField { start: 294, len: 8 }) / 10.0,
        destination: get(&bits, BitField { start: 302, len: 120 }),
    }
}
