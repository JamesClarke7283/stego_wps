use criterion::{black_box, Criterion};
use stego_wps::encode;

pub fn bench_encode(c: &mut Criterion) {
    c.bench_function("encode", |b| {
        b.iter(|| {
            encode(black_box(
                "This is a test sentence. Followed by another test sentence.",
            ))
        });
    });
}
