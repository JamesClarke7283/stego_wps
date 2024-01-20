use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

mod compare;
mod decode;
mod encode;

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(1000);
    targets =
        compare::bench_compare_small,
        compare::bench_compare_medium,
        compare::bench_compare_large,
        decode::bench_decode_small,
        decode::bench_decode_medium,
        decode::bench_decode_large,
        encode::bench_encode_small,
        encode::bench_encode_medium,
        encode::bench_encode_large
);
criterion_main!(benches);
