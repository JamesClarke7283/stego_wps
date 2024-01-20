use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stego_wps::compare;

pub fn bench_compare_small(c: &mut Criterion) {
    let secret_message = "HELLO";
    let cover_text = "This is ort.";
    let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    c.bench_function("compare_small", |b| {
        b.iter(|| {
            compare(
                black_box(secret_message),
                black_box(cover_text),
                black_box(character_set),
            )
        });
    });
}

pub fn bench_compare_medium(c: &mut Criterion) {
    let secret_message = "HELLO WORLD";
    let cover_text = "This is a bit longer text to compare. It ould take a bit longer.";
    let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    c.bench_function("compare_medium", |b| {
        b.iter(|| {
            compare(
                black_box(secret_message),
                black_box(cover_text),
                black_box(character_set),
            )
        });
    });
}

pub fn bench_compare_large(c: &mut Criterion) {
    let secret_message = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";
    let cover_text = "This is a much much longer text to compare. It is a full sentence that is complex enough for a good benchmark test. This ould ideally take the longest time to compare among the three.";
    let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    c.bench_function("compare_large", |b| {
        b.iter(|| {
            compare(
                black_box(secret_message),
                black_box(cover_text),
                black_box(character_set),
            )
        });
    });
}

criterion_group!(benches, bench_compare_small, bench_compare_medium, bench_compare_large);
criterion_main!(benches);
