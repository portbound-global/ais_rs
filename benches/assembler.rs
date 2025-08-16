use ais_rs::assembler::Assembler;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_process_partial_sentence(c: &mut Criterion) {
    let mut assembler = Assembler::new();
    let messages = [
        "!AIVDM,2,1,8,B,53ku7W82=OJd=4i<000DhhDr0E=<8E8LE800001I<h@9:5f<0@0Q@CTP,0*38",
        "!AIVDM,2,2,8,B,000000000000000,2*2F",
    ];

    c.bench_function("process_partial_sentence", |b| {
        b.iter(|| {
            for msg in messages {
                let _ = assembler.push(msg);
            }
        })
    });
}

criterion_group!(benches, bench_process_partial_sentence);
criterion_main!(benches);
