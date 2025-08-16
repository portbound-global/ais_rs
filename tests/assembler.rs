#[cfg(test)]
mod tests {
    use ais_rs::assembler::Assembler;
    use ais_rs::log_error_sentence;
    use ais_rs::models::BuildSentence;

    #[test]
    fn test_assembler() {
        let mut results = Vec::<BuildSentence>::new();

        let mut assembler = Assembler::new();

        let messages = [
            "!AIVDM,2,1,8,B,51ku7W82=OJd=4i<000DhhDr0E=<8E8LE800001I<h@9:5f<0@0Q@CTP,0*38",
            "!AIVDM,2,2,8,B,000000000000000,2*2F",
        ];

        for msg in messages {
            let res = assembler.push(msg);

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
        // assert_eq!(results.len(), 1);
    }
}
