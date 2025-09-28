use crate::models::{AISMessage, AISPartial, PartialPositionClassA, PartialStaticData, BuildSentence};
use crate::parser::aid_to_navigation::aid_to_navigation;
use crate::parser::bit_parser::{get, BitField};
use crate::parser::position_class_a::position_class_a;
use crate::parser::static_data::static_data;

impl BuildSentence {
    fn payload_to_bit_vector(&self, take_amount: usize) -> Vec<u8> {
        let chars = self.payload.chars();

        let capacity = (take_amount * 6) + self.fill_bits as usize;

        let mut bits_vector:Vec<u8> = Vec::with_capacity(capacity);

        for char in chars.take(take_amount) {
            let mut value = char as u8;
            value -= if value > 88 { 56 } else { 48 };

            // Push 6 bits as separate u8s
            for i in (0..6).rev() {
                bits_vector.push((value >> i) & 1);
            }
        }

        for _ in 0..self.fill_bits {
            bits_vector.push(0);
        }

        bits_vector
    }

    #[inline(always)]
    pub fn decode(&self) -> AISMessage {
        let bits_vector = self.payload_to_bit_vector(self.payload.len());

        let msg_type = get(&bits_vector, BitField { start: 0, len: 6 });

        match msg_type {
            1 | 2 | 3 => {
                let decoded_message = position_class_a(bits_vector, msg_type);
                AISMessage::Position(decoded_message)
            }
            5 => {
                let decoded_message = static_data(bits_vector, msg_type);
                AISMessage::Static(decoded_message)
            }
            21 => {
                let decoded_message = aid_to_navigation(bits_vector, msg_type);
                AISMessage::NotN(decoded_message)
            }
            _ => AISMessage::Unknown(msg_type)
        }
    }

    #[inline(always)]
    pub fn partial_decode(&self) -> AISPartial {
        let mut payload_chars = self.payload.chars();

        if let Some(char) = payload_chars.next() {
            let mut msg_type = char as u8;
            msg_type -= if msg_type > 88 { 56 } else { 48 };

            return match msg_type {
                1 | 2 | 3 => {
                    let bit_vector = self.payload_to_bit_vector(7);

                    let decoded_message = PartialPositionClassA {
                        mmsi: get(&bit_vector, BitField { start: 8, len: 30 }),
                    };

                    AISPartial::Position(decoded_message)
                }
                5 => {
                    let bit_vector = self.payload_to_bit_vector(40);

                    let decoded_message = PartialStaticData {
                        mmsi: get(&bit_vector, BitField { start: 8, len: 30 }),
                        imo: get(&bit_vector, BitField { start: 40, len: 30 }),
                        ship_type: get(&bit_vector, BitField { start: 232, len: 8 }),
                    };

                    AISPartial::Static(decoded_message)
                }
                _ => AISPartial::Unknown(msg_type)
            }
        }
        AISPartial::Unknown(0)
    }
}