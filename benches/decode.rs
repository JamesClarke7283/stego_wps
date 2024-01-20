use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stego_wps::decode;

pub fn bench_decode_small(c: &mut Criterion) {
    let encoded = vec![5, 7];
    let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    c.bench_function("decode_small", |b| {
        b.iter(|| decode(black_box(&encoded), black_box(character_set)));
    });
}

pub fn bench_decode_medium(c: &mut Criterion) {
    let encoded = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]; // Medium-length vector (Fibonacci sequence truncated to 11 elements)
    let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    c.bench_function("decode_medium", |b| {
        b.iter(|| decode(black_box(&encoded), black_box(character_set)));
    });
}

pub fn bench_decode_large(c: &mut Criterion) {
    let encoded = (1..50).collect::<Vec<usize>>(); // Large-length vector from 1 to 50
    let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    c.bench_function("decode_large", |b| {
        b.iter(|| decode(black_box(&encoded), black_box(character_set)));
    });
}

criterion_group!(benches, bench_decode_small, bench_decode_medium, bench_decode_large);
criterion_main!(benches);

