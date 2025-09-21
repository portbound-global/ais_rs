#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use ais_rs::assembler::{MultipartAssembler};
    use ais_rs::log_error_sentence;
    use ais_rs::models::{AISMessage, AISPartial};

    #[test]
    fn decode_full() {
        let mut assembler = MultipartAssembler::new();

        let signals = read_to_string("./ais.log").unwrap();

        for signal in signals.split('\n') {
            let assembled_msg = match assembler.push(signal) {
                Ok(msg) => msg,
                Err(err) => {
                    log_error_sentence!(err, signal);
                    continue
                },
            };

            if let Some(message) = assembled_msg {
                match message.decode() {
                    AISMessage::Position(message) => {
                        println!("{:#?}", message);
                    }
                    AISMessage::Static(message) => {
                        println!("{:#?}", message);
                    }
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn decode_partial() {
        let mut assembler = MultipartAssembler::new();

        let signals = read_to_string("./ais.log").unwrap();

        for signal in signals.split('\n') {
            let assembled_msg = match assembler.push(signal) {
                Ok(msg) => msg,
                Err(err) => {
                    log_error_sentence!(err, signal);
                    continue
                },
            };

            if let Some(message) = assembled_msg {
                match message.partial_decode() {
                    AISPartial::Position(message) => {
                        println!("{:#?}", message);
                    }
                    AISPartial::Static(message) => {
                        println!("{:#?}", message);
                    }
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn decode() {
        let mut assembler = MultipartAssembler::new();

        let signals = read_to_string("./ais.log").unwrap();
        for signal in signals.split('\n') {
            let assembled_msg = match assembler.push(signal) {
                Ok(msg) => msg,
                Err(err) => {
                    log_error_sentence!(err, signal);
                    continue
                },
            };

            if let Some(message) = assembled_msg {
                match (message.decode(), message.partial_decode()) {
                    (AISMessage::Static(full), AISPartial::Static(partial)) => {
                        assert_eq!(full.mmsi, partial.mmsi);
                        assert_eq!(full.imo, partial.imo);
                        assert_eq!(full.ship_type, partial.ship_type);

                        println!("Partial and fully decoded messages match: (MMSI: {})", full.mmsi);
                    }
                    (AISMessage::Position(full), AISPartial::Position(partial)) => {
                        assert_eq!(full.mmsi, partial.mmsi);
                        println!("Partial and fully decoded messages match: (MMSI: {})", full.mmsi);
                    }
                    (_, _) => {}
                }
            }
        }
    }
}
