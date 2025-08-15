use crate::errors::AisError;
use crate::models::NmeaSentence;

const MESSAGE_START_CHAR: char = '!';
const CHECKSUM_DELIMITER: char = '*';
const FIELD_DELIMITER: char = ',';
const MIN_FIELD_COUNT: usize = 7;

fn validate_and_strip_checksum(sentence: &str) -> Result<&str, AisError> {
    let sentence = sentence
        .strip_prefix(MESSAGE_START_CHAR)
        .ok_or_else(|| AisError::MalformedSentence("Missing start character".to_string()))?;

    let (data, checksum) = sentence
        .split_once(CHECKSUM_DELIMITER)
        .ok_or_else(|| AisError::MalformedSentence("Missing checksum delimiter".to_string()))?;

    let expected_checksum =
        u8::from_str_radix(checksum, 16).map_err(|_| AisError::TypeConversion {
            field: "checksum".to_string(),
            value: checksum.to_string(),
        })?;

    let calculated_checksum = data.bytes().fold(0u8, |acc, x| acc ^ x);

    if calculated_checksum == expected_checksum {
        Ok(data)
    } else {
        Err(AisError::InvalidChecksum {
            expected: expected_checksum,
            found: calculated_checksum,
        })
    }
}

pub fn structurize_sentence(sentence: &str) -> Result<NmeaSentence, AisError> {
    let data = validate_and_strip_checksum(sentence)?;

    let fields: Vec<&str> = data.split(FIELD_DELIMITER).collect();

    if fields.len() < MIN_FIELD_COUNT {
        return Err(AisError::MalformedSentence(
            "Sentence is missing Fields".to_string(),
        ));
    }

    // Start parsing the fields into variables with error handling.
    let talker = fields[0].to_string();

    let total_sentences = fields[1].parse().map_err(|_| AisError::TypeConversion {
        field: "total_sentences".to_string(),
        value: fields[1].to_string(),
    })?;

    let sentence_number = fields[2].parse().map_err(|_| AisError::TypeConversion {
        field: "sentence_number".to_string(),
        value: fields[2].to_string(),
    })?;

    let sentence_id = fields[3].parse().ok();

    let channel = fields[4].chars().next().ok_or(AisError::TypeConversion {
        field: "channel".to_string(),
        value: fields[4].to_string(),
    })?;

    let payload = fields[5].to_string();

    let fill_bits = fields[6].parse().map_err(|_| AisError::TypeConversion {
        field: "fill_bits".to_string(),
        value: fields[6].to_string(),
    })?;

    // Last check if the sentence number is correct.
    if sentence_number == 0 || sentence_number > total_sentences {
        let error_message = format!(
            "sentence_number ({}) out of range for total_sentences ({})",
            sentence_number, total_sentences
        );

        return Err(AisError::MalformedSentence(error_message));
    }

    Ok(NmeaSentence {
        talker,
        total_sentences,
        sentence_number,
        sentence_id,
        channel,
        payload,
        fill_bits,
    })
}
