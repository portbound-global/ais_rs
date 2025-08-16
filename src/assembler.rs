use crate::errors::AISError;
use crate::models::{BuildSentence, NmeaSentence, PartialSentence};
use crate::nmea;
use std::{collections::HashMap, time::Instant};

pub struct MultipartAssembler {
    sentence_hashmap: HashMap<(u8, char), PartialSentence>,
}

impl MultipartAssembler {
    pub fn new() -> Self {
        MultipartAssembler {
            sentence_hashmap: HashMap::new(),
        }
    }
    fn process_partial_sentence(&mut self, sentence: NmeaSentence) -> Option<BuildSentence> {
        let seq_id = (sentence.sentence_id.unwrap(), sentence.channel);
        let sentence_number = sentence.sentence_number;
        let total_sentences = sentence.total_sentences;

        let idx = (sentence_number - 1) as usize;

        {
            let group = self
                .sentence_hashmap
                .entry(seq_id)
                .or_insert_with(|| PartialSentence {
                    total_sentences,
                    payloads: vec![String::with_capacity(64); total_sentences as usize],
                    fill_bits: 0,
                    payload_len: 0,
                    available: false,
                });

            if group.available {
                group.total_sentences = total_sentences;
                group
                    .payloads
                    .resize(total_sentences as usize, String::with_capacity(64));
                group.payload_len = 0;
                group.available = false;
            }

            group.payloads[idx] = sentence.payload;
            group.payload_len += 1;

            if sentence_number == group.total_sentences {
                group.fill_bits = sentence.fill_bits;
            }
        }

        if let Some(group) = self.sentence_hashmap.get_mut(&seq_id) {
            if group.payload_len == group.total_sentences {
                let full_payload = group.payloads.join("");

                let build_sentence = BuildSentence {
                    talker: sentence.talker,
                    channel: sentence.channel,
                    payload: full_payload,
                    fill_bits: group.fill_bits,
                    timestamp: Instant::now(),
                };

                // Make the group available again to avoid memory reallocation.
                group.available = true;

                return Some(build_sentence);
            }
        }

        None
    }

    pub fn push(&mut self, sentence: &str) -> Result<Option<BuildSentence>, AISError> {
        let sentence = nmea::structurize_sentence(&sentence);

        match sentence {
            Ok(sentence) => {
                if sentence.total_sentences == 1 {
                    let build_sentence = BuildSentence {
                        talker: sentence.talker,
                        channel: sentence.channel,
                        payload: sentence.payload,
                        fill_bits: sentence.fill_bits,
                        timestamp: Instant::now(),
                    };

                    Ok(Some(build_sentence))
                } else {
                    let full_sentence = self.process_partial_sentence(sentence);
                    Ok(full_sentence)
                }
            }
            Err(error) => Err(error),
        }
    }
}

pub fn single_part_assembler(sentence: &str) -> Result<Option<BuildSentence>, AISError> {
    let sentence = nmea::structurize_sentence(&sentence)?;

    if sentence.total_sentences == 1 {
        let build_sentence = BuildSentence {
            talker: sentence.talker,
            channel: sentence.channel,
            payload: sentence.payload,
            fill_bits: sentence.fill_bits,
            timestamp: Instant::now(),
        };

        Ok(Some(build_sentence))
    } else {
        Err(AISError::IsMultipartSentence {
            current_part: sentence.sentence_number,
            total_parts: sentence.total_sentences,
        })
    }
}
