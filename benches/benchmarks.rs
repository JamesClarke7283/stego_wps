use criterion::{criterion_group, criterion_main};

pub mod compare;
pub mod decode;
pub mod encode;

criterion_group!(
    benches,
    encode::bench_encode,
    decode::bench_decode,
    compare::bench_compare
);
criterion_main!(benches);
