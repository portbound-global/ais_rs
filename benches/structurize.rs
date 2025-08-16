use ais_rs::nmea::structurize_sentence;
use criterion::{criterion_group, criterion_main, Criterion};

fn structurize_message(c: &mut Criterion) {
    let messages = [
        "!AIVDM,2,1,8,B,53ku7W82=OJd=4i<000DhhDr0E=<8E8LE800001I<h@9:5f<0@0Q@CTP,0*38",
        "!AIVDM,2,2,8,B,000000000000000,2*2F",
    ];

    c.bench_function("structurize_message", |b| {
        b.iter(|| {
            for msg in messages {
                let _ = structurize_sentence(msg);
            }
        })
    });
}

criterion_group!(benches, structurize_message);
criterion_main!(benches);
