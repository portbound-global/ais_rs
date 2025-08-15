#[cfg(test)]
mod tests {
    use ais_rs::assembler::Assembler;
    use ais_rs::log_error_sentence;
    use ais_rs::models::NmeaSentence;

    #[test]
    fn test_assembler() {
        let mut results = Vec::<NmeaSentence>::new();

        let mut assembler = Assembler::new();

        let messages = [
            "!AIVDM,2,1,B,53ku7W82=OJd=4i<000DhhDr0E=<8E8LE800001I<h@9:5f<0@0Q@CTP,0*38",
            "!AIVDM,1,1,,A,13`mgEP0000A0CRML>1GjWSn0@0W,0*65",
        ];

        for msg in messages {
            let res = assembler.push(msg.to_string());

            match res {
                Ok(sentence) => {
                    if let Some(sentence) = sentence {
                        println!("{:#?}", sentence);
                        results.push(sentence);
                    }
                }
                Err(e) => {
                    log_error_sentence!(e, msg)
                }
            }
        }
        assert_eq!(results.len(), 1);
    }
}
