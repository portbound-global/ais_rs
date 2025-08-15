use crate::errors::AisError;
use crate::models::{NmeaSentence, SentenceGroup};
use crate::nmea;
use std::collections::HashMap;
use std::time::{Duration, Instant};

const PARTIAL_SENTENCE_TIMEOUT: Duration = Duration::from_secs(30);

pub struct Assembler {
    sentence_hashmap: HashMap<u8, SentenceGroup>,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            sentence_hashmap: HashMap::new(),
        }
    }

    fn cleanup_stale(&mut self) {
        let now = Instant::now();
        self.sentence_hashmap
            .retain(|_key, group| now.duration_since(group.created_at) < PARTIAL_SENTENCE_TIMEOUT);
    }

    fn process_partial_sentence(&mut self, _sentence: NmeaSentence) -> Option<NmeaSentence> {
        // TODO: sentence construction
        None
    }

    pub fn push(&mut self, sentence: String) -> Result<Option<NmeaSentence>, AisError> {
        self.cleanup_stale();
        let sentence = nmea::structurize_sentence(&sentence);

        match sentence {
            Ok(sentence) => {
                if sentence.total_sentences == 1 {
                    Ok(Some(sentence))
                } else {
                    let full_sentence = self.process_partial_sentence(sentence);

                    Ok(full_sentence)
                }
            }
            Err(error) => Err(error),
        }
    }
}
