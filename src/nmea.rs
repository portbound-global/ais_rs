const MESSAGE_START_CHAR: char = '!';
const CHECKSUM_DELIMITER: char = '*';

pub fn is_checksum_valid(sentence: &str) -> bool {
    let Some(sentence) = sentence.strip_prefix(MESSAGE_START_CHAR) else {
        return false;
    };

    let Some((data, checksum)) = sentence.split_once(CHECKSUM_DELIMITER) else {
        return false;
    };

    let expected_checksum = match u8::from_str_radix(checksum, 16) {
        Ok(val) => val,
        Err(_) => return false,
    };

    let calculated_checksum = data.bytes().fold(0u8, |acc, x| acc ^ x);

    calculated_checksum == expected_checksum
}
