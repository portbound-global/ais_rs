use ais_rs::assembler::single_part_assembler;
use criterion::{criterion_group, criterion_main, Criterion};
use ais_rs::decoder::full::decode_ais_sentence;

fn benchmark_full_decoding(c: &mut Criterion) {
    let signal = "!AIVDM,1,1,,A,13`mgEP0000A0CRML>1GjWSn0@0W,0*65";

    c.bench_function("Single part full-decoding (benchmark)", |b| {
        b.iter(|| {
            let assembled_msg = single_part_assembler(signal);

            let decoded = decode_ais_sentence(assembled_msg, Some(signal));

            if let Some(_message) = decoded {}
        })
    });
}

criterion_group!(benches, benchmark_full_decoding);
criterion_main!(benches);
