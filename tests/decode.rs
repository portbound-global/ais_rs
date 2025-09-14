#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use ais_rs::assembler::{single_part_assembler, MultipartAssembler};
    use ais_rs::decoder::decode_ais_sentence;
    use ais_rs::models::{AISMessage};

    const SIGNALS: [&str; 1] = [
        "!AIVDM,1,1,,A,402;2L1vVCe2DP8NCdM:aNk02@;p,0*2F",
    ];

    #[test]
    fn decode() {
        let mut assembler = MultipartAssembler::new();

        let signals = read_to_string("./ais.log").unwrap();

        for signal in signals.split('\n') {
            let assembled_msg = assembler.push(signal);

            if let Some(message) = decode_ais_sentence(assembled_msg, Some(signal)) {
                match message {
                    AISMessage::Position(message) => {
                    }
                    AISMessage::Static(message) => {
                        println!("{:#?}", message.destination);
                    }
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn decode_single() {
        let signal = SIGNALS[0];

        let assembled_msg = single_part_assembler(signal);

        assert!(assembled_msg.is_ok());

        let decoded = decode_ais_sentence(assembled_msg, Some(signal));

        assert!(decoded.is_some());

        if let Some(message) = decoded {
            println!("{:#?}", message);
        }
    }
}
