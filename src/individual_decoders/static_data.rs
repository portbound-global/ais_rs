use crate::individual_decoders::bit_parser::{get, BitField};
use crate::models::AISStaticData;

pub fn static_data(bits: Vec<u8>, _message_type: u8) -> AISStaticData {
    let ship_type = 0;
    let dimension_to_bow = 0;
    let dimension_to_stern = 0;
    let dimension_to_port = 0;
    let dimension_to_starboard = 0;

    AISStaticData {
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
        ship_type,
        dimension_to_bow,
        dimension_to_stern,
        dimension_to_port,
        dimension_to_starboard,
    }
}
