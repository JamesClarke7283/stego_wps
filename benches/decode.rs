use criterion::{black_box, Criterion};
use stego_wps::decode;

pub fn bench_decode(c: &mut Criterion) {
    let encoded = vec![5, 7];
    let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    c.bench_function("decode", |b| {
        b.iter(|| decode(black_box(&encoded), black_box(character_set)));
    });
}
