#[cfg(test)]
mod tests {
    use ais_rs::nmea::structurize_sentence;

    #[test]
    fn structurize() {
        // Valid message
        let result = structurize_sentence("!AIVDM,1,1,,A,13`mgEP0000A0CRML>1GjWSn0@0W,0*65");
        assert!(result.is_ok());

        // Invalid checksum
        let result = structurize_sentence("!AIVDM,1,1,,A,13`mgEP0000A0CRML>1GjWSn0@0W,0*64");
        assert!(result.is_err());

        // Malformed payload
        let result = structurize_sentence("!AIVDM,1,1,,A,13`mgEP0000A0CRM>1GjWSn0@0W,0*65");
        assert!(result.is_err());

        // Missing start char
        let result = structurize_sentence("AIVDM,1,1,,A,13`mgEP0000A0CRML>1GjWSn0@0W,0*65");
        assert!(result.is_err());

        // Missing field
        let result = structurize_sentence("!AIVDM,1,1,,13`mgEP0000A0CRML>1GjWSn0@0W,0*65");
        assert!(result.is_err());

        // Wrong sentence number
        let result = structurize_sentence("!AIVDM,1,0,,A,13`mgEP0000A0CRML>1GjWSn0@0W,0*65");
        assert!(result.is_err());
    }
}
