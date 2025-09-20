use std::str::Chars;
use crate::errors::AISError;
use crate::parser::bit_parser::{get, BitField};
use crate::log_error_sentence;
use crate::models::{AISPartial, AISPartialPositionClassA, AISPartialStaticData, BuildSentence};

pub fn decode_partial_ais_sentence(
    assembled_msg: Result<Option<BuildSentence>, AISError>,
    signal: Option<&str>,
) -> Option<AISPartial> {

    match assembled_msg {
        Ok(msg) => {
            if let Some(message) = msg {
                return decode(message);
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

fn payload_to_bit_vector(chars: Chars, take_amount: usize) -> Vec<u8> {
    let mut bits_vector:Vec<u8> = Vec::with_capacity(take_amount * 6);

    for char in chars.take(take_amount) {
        let mut value = char as u8;
        value -= if value > 88 { 56 } else { 48 };

        // Push 6 bits as separate u8s
        for i in (0..6).rev() {
            bits_vector.push((value >> i) & 1);
        }
    }

    bits_vector
}

fn decode(sentence: BuildSentence) -> Option<AISPartial> {
    let payload = sentence.payload;
    let mut payload_chars = payload.chars();

    if let Some(char) = payload_chars.next() {
        let mut msg_type = char as u8;
        msg_type -= if msg_type > 88 { 56 } else { 48 };

        return match msg_type {
            1 | 2 | 3 => {
                // take 6 chars since the first is already consumed
                let bit_vector = payload_to_bit_vector(payload_chars, 6);

                let decoded_message = AISPartialPositionClassA {
                    mmsi: get(&bit_vector, BitField { start: 2, len: 30 }),
                };

                Some(AISPartial::Position(decoded_message))
            }
            5 => {
                // take 6 chars since the first is already consumed
                let bit_vector = payload_to_bit_vector(payload_chars, 39);

                let decoded_message = AISPartialStaticData {
                    mmsi: get(&bit_vector, BitField { start: 2, len: 30 }),
                    imo: get(&bit_vector, BitField { start: 34, len: 30 }),
                    ship_type: get(&bit_vector, BitField { start: 226, len: 8 }),
                };

                Some(AISPartial::Static(decoded_message))
            }
            _ => Some(AISPartial::Unknown(msg_type))
        }
    }
    None
}
