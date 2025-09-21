#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use ais_rs::assembler::{MultipartAssembler};
    use ais_rs::decoder::full::decode_ais_sentence;
    use ais_rs::decoder::partial::decode_partial_ais_sentence;
    use ais_rs::models::{AISMessage, AISPartial};

    #[test]
    fn decode_full() {
        let mut found_destinations:Vec<String> = Vec::new();
        let mut assembler = MultipartAssembler::new();

        let signals = read_to_string("./ais.log").unwrap();

        for signal in signals.split('\n') {
            let assembled_msg = assembler.push(signal);

            if let Some(message) = decode_ais_sentence(assembled_msg, Some(signal)) {
                match message {
                    // AISMessage::Position(message) => {}
                    AISMessage::Static(message) => {
                        let destination = message.destination.to_lowercase();

                        if !found_destinations.contains(&destination) {
                            println!("{}", destination);
                            found_destinations.push(destination);
                        }
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
            let assembled_msg = assembler.push(signal);

            if let Some(message) = decode_partial_ais_sentence(assembled_msg, Some(signal)) {
                match message {
                    AISPartial::Position(message) => {
                        // println!("{:#?}", message);
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
            let assembled_msg = assembler.push(signal);

            let full_message = decode_ais_sentence(assembled_msg.clone(), Some(signal));
            let partial_message = decode_partial_ais_sentence(assembled_msg, Some(signal));

            match (full_message, partial_message) {
                (Some(full), Some(partial)) => {

                    match (full, partial) {
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
                _ => {}
            }
        }
    }
}
