use crate::errors::AISError;
use crate::individual_decoders::bit_parser;
use crate::individual_decoders::bit_parser::BitField;
use crate::individual_decoders::position_class_a::position_class_a;
use crate::individual_decoders::static_data::static_data;
use crate::log_error_sentence;
use crate::models::{AISMessage, BuildSentence};

pub fn decode_ais_sentence(
    assembled_msg: Result<Option<BuildSentence>, AISError>,
    signal: Option<&str>,
) -> Option<AISMessage> {
    match assembled_msg {
        Ok(msg) => {
            if let Some(message) = msg {
                return Some(decode(message));
            };
            None
        }
        Err(e) => {
            if let Some(signal) = signal {
                log_error_sentence!(e, signal);
            }
            None
        }
    }
}

fn decode(sentence: BuildSentence) -> AISMessage {
    let payload = sentence.payload;

    let mut bits_vector = Vec::with_capacity(payload.len() * 6);

    for char in payload.chars() {
        let mut value = char as u8;
        value -= if value > 88 { 56 } else { 48 };

        // Push 6 bits as separate u8s
        for i in (0..6).rev() {
            bits_vector.push((value >> i) & 1);
        }
    }

    let msg_type = bit_parser::get(&bits_vector, BitField { start: 0, len: 6 });

    match msg_type {
        1 | 2 | 3 => {
            let decoded_message = position_class_a(bits_vector, msg_type);
            AISMessage::Position(decoded_message)
        }
        5 => {
            let decoded_message = static_data(bits_vector, msg_type);
            AISMessage::Static(decoded_message)
        }
        _ => AISMessage::Unknown(msg_type),
    }
}
