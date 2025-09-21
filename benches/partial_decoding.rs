use ais_rs::assembler::single_part_assembler;
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_partial_decoding(c: &mut Criterion) {
    let signal = "!AIVDM,1,1,,A,13`mgEP0000A0CRML>1GjWSn0@0W,0*65";

    c.bench_function("Single part partial-decoding (benchmark)", |b| {
        b.iter(|| {
            let assembled_msg = single_part_assembler(signal).expect("Single part assembly failed");

            assembled_msg.partial_decode()
        })
    });
}

criterion_group!(benches, benchmark_partial_decoding);
criterion_main!(benches);
