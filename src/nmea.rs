use crate::errors::AISError;
use crate::models::NmeaSentence;

const MESSAGE_START_CHAR: char = '!';
const CHECKSUM_DELIMITER: char = '*';
const FIELD_DELIMITER: char = ',';
const MIN_FIELD_COUNT: usize = 7;

fn validate_and_strip_checksum(sentence: &str) -> Result<&str, AISError> {
    let sentence = sentence
        .strip_prefix(MESSAGE_START_CHAR)
        .ok_or_else(|| AISError::MalformedSentence("Missing start character".to_string()))?;

    let (data, checksum) = sentence
        .split_once(CHECKSUM_DELIMITER)
        .ok_or_else(|| AISError::MalformedSentence("Missing checksum delimiter".to_string()))?;

    let expected_checksum =
        u8::from_str_radix(checksum, 16).map_err(|_| AISError::TypeConversion {
            field: "checksum".to_string(),
            value: checksum.to_string(),
        })?;

    let calculated_checksum = data.bytes().fold(0u8, |acc, x| acc ^ x);

    if calculated_checksum == expected_checksum {
        Ok(data)
    } else {
        Err(AISError::InvalidChecksum {
            expected: expected_checksum,
            found: calculated_checksum,
        })
    }
}

pub fn structurize_sentence(sentence: &str) -> Result<NmeaSentence, AISError> {
    let data = validate_and_strip_checksum(sentence)?;

    let mut fields = data.splitn(MIN_FIELD_COUNT, FIELD_DELIMITER);

    let talker = fields.next().unwrap_or_default().to_string();

    let total_sentences: u8 = fields
        .next()
        .ok_or_else(|| AISError::MalformedSentence("Missing total_sentences".into()))?
        .parse()
        .map_err(|_| AISError::TypeConversion {
            field: "total_sentences".into(),
            value: fields.clone().next().unwrap_or_default().to_string(),
        })?;

    let sentence_number: u8 = fields
        .next()
        .ok_or_else(|| AISError::MalformedSentence("Missing sentence_number".into()))?
        .parse()
        .map_err(|_| AISError::TypeConversion {
            field: "sentence_number".into(),
            value: fields.clone().next().unwrap_or_default().to_string(),
        })?;

    let sentence_id: Option<u8> = fields.next().and_then(|s| s.parse().ok());

    let channel = fields
        .next()
        .and_then(|s| s.chars().next())
        .ok_or_else(|| AISError::TypeConversion {
            field: "channel".into(),
            value: "".into(),
        })?;

    let payload = fields
        .next()
        .ok_or_else(|| AISError::MalformedSentence("Missing payload".into()))?
        .to_string();

    let fill_bits: u8 = fields
        .next()
        .ok_or_else(|| AISError::MalformedSentence("Missing fill_bits".into()))?
        .parse()
        .map_err(|_| AISError::TypeConversion {
            field: "fill_bits".into(),
            value: "".into(),
        })?;

    if sentence_number == 0 || sentence_number > total_sentences {
        return Err(AISError::MalformedSentence(format!(
            "sentence_number ({}) out of range for total_sentences ({})",
            sentence_number, total_sentences
        )));
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
