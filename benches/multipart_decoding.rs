use ais_rs::assembler::MultipartAssembler;
use ais_rs::decoder::decode_ais_sentence;
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_multipart_decoding(c: &mut Criterion) {
    let mut assembler = MultipartAssembler::new();

    let messages = [
        "!AIVDM,2,1,8,B,53ku7W82=OJd=4i<000DhhDr0E=<8E8LE800001I<h@9:5f<0@0Q@CTP,0*38",
        "!AIVDM,2,2,8,B,000000000000000,2*2F",
    ];

    c.bench_function("Multipart decoding (benchmark)", |b| {
        b.iter(|| {
            for signal in messages {
                let assembled_msg = assembler.push(signal);

                if let Some(_message) = decode_ais_sentence(assembled_msg, Some(signal)) {}
            }
        })
    });
}

criterion_group!(benches, benchmark_multipart_decoding);
criterion_main!(benches);
