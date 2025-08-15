#[cfg(test)]
mod tests {
    use ais_rs::nmea;

    #[test]
    fn checksum() {
        assert!(nmea::is_checksum_valid(
            "!AIVDM,1,1,,A,13`mgEP0000A0CRML>1GjWSn0@0W,0*65"
        )); // valid message
        assert!(!nmea::is_checksum_valid(
            "!AIVDM,1,1,,A,13`mgEP0000A0CRML>1GjWSn0@0W,0*64"
        )); // invalid checksum
        assert!(!nmea::is_checksum_valid(
            "!AIVDM,1,1,,A,13`mgEP0000A0CRML>1GjWSn0@0w,0*65"
        )); // invalid message payload
        assert!(!nmea::is_checksum_valid(
            "AIVDM,1,1,,A,13`mgEP0000A0CRML>1GjWSn0@0W,0*65"
        )); // invalid message start
    }
}
