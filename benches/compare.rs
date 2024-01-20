use criterion::{black_box, Criterion};
use stego_wps::compare;

pub fn bench_compare(c: &mut Criterion) {
    let secret_message = "HELLO";
    let cover_text = "This is a test sentence. Followed by another test sentence.";
    let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    c.bench_function("compare", |b| {
        b.iter(|| {
            compare(
                black_box(secret_message),
                black_box(cover_text),
                black_box(character_set),
            )
        });
    });
}
