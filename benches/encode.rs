use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stego_wps::encode;

pub fn bench_encode_small(c: &mut Criterion) {
    let text = "Short sentence.";
    c.bench_function("encode_small", |b| {
        b.iter(|| encode(black_box(text)))
    });
}

pub fn bench_encode_medium(c: &mut Criterion) {
    let text = "This is a medium length paragraph. It contains multiple sentences. They serve to create a more substantial encode workload.";
    c.bench_function("encode_medium", |b| {
        b.iter(|| encode(black_box(text)))
    });
}

pub fn bench_encode_large(c: &mut Criterion) {
    let text = "This is a much longer text intended for the large benchmark of the encode function. It spans several sentences and aims to represent a more massive workload typical in extensive encoding operations. The complexity and length are intentionally higher to test the performance impact on larger inputs.";
    c.bench_function("encode_large", |b| {
        b.iter(|| encode(black_box(text)))
    });
}

criterion_group!(benches, bench_encode_small, bench_encode_medium, bench_encode_large);
criterion_main!(benches);

