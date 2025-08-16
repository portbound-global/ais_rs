#[cfg(test)]
mod tests {
    use ais_rs::assembler::{single_part_assembler, MultipartAssembler};
    use ais_rs::decoder::decode_ais_sentence;

    const SIGNALS: [&str; 3] = [
        "!AIVDM,1,1,,A,13`mgEP0000A0CRML>1GjWSn0@0W,0*65",
        "!AIVDM,2,1,2,A,59NSJ@p2=vcPCDI;V20<PU5DU@61A84@E:22221?H0DG75e<0F3S5S2H,0*1B",
        "!AIVDM,2,2,2,A,888888888888880,2*26",
    ];

    #[test]
    fn decode() {
        let mut assembler = MultipartAssembler::new();

        for signal in SIGNALS {
            let assembled_msg = assembler.push(signal);

            if let Some(message) = decode_ais_sentence(assembled_msg, Some(signal)) {
                println!("{:#?}", message)
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
