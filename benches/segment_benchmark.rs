use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sentencex::segment; // Replace `sentencex` with the actual crate name if different

fn benchmark_segment(c: &mut Criterion) {
    let text = "This is a sentence. Here is another one. And yet another sentence follows.";
    c.bench_function("segment_english", |b| {
        b.iter(|| segment(black_box("en"), black_box(text)))
    });
}

criterion_group!(benches, benchmark_segment);
criterion_main!(benches);
