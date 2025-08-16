use ais_rs::assembler::single_part_assembler;
use ais_rs::decoder::decode_ais_sentence;
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_single_part_decoding(c: &mut Criterion) {
    let signal = "!AIVDM,1,1,,A,13`mgEP0000A0CRML>1GjWSn0@0W,0*65";

    c.bench_function("Signle part decoding (benchmark)", |b| {
        b.iter(|| {
            let assembled_msg = single_part_assembler(signal);

            let decoded = decode_ais_sentence(assembled_msg, Some(signal));

            if let Some(_message) = decoded {}
        })
    });
}

criterion_group!(benches, benchmark_single_part_decoding);
criterion_main!(benches);
