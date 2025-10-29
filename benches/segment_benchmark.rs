use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use sentencex::{get_sentence_boundaries, segment};
use std::hint::black_box;

fn benchmark_segment(c: &mut Criterion) {
    let text = "This is a sentence. Here is another one. And yet another sentence follows.";
    c.bench_function("segment_english", |b| {
        b.iter(|| segment(black_box("en"), black_box(text)))
    });
}
// Generate text of varying sizes for benchmarking
fn generate_text(size: usize) -> String {
    let base = "This is a sentence. Here is another one. And yet another sentence follows. \
               Dr. Smith works at the U.S. National Institute of Health. \
               The meeting is scheduled for 3 p.m. on Jan. 15th. \
               She said, \"This is important!\" and left. ";

    let mut text = String::with_capacity(size);
    while text.len() < size {
        text.push_str(base);
    }
    text.truncate(size);
    text
}

fn bench_segment_by_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("segment_by_size");

    // Reduced max size to avoid timeouts, focus on realistic use cases
    for size in [100, 1_000, 10_000, 50_000].iter() {
        let text = generate_text(*size);
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &text, |b, text| {
            b.iter(|| segment(black_box("en"), black_box(text)))
        });
    }

    group.finish();
}

fn bench_boundary_detection_by_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("boundary_detection_by_size");

    for size in [100, 1_000, 10_000, 50_000].iter() {
        let text = generate_text(*size);
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &text, |b, text| {
            b.iter(|| get_sentence_boundaries(black_box("en"), black_box(text)))
        });
    }

    group.finish();
}

fn bench_multi_language(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_language");

    let languages = vec![
        (
            "en",
            "This is an English sentence. Another one follows. And yet another!",
        ),
        (
            "fr",
            "Ceci est une phrase en français. Une autre suit. Et encore une autre!",
        ),
        (
            "de",
            "Dies ist ein deutscher Satz. Ein weiterer folgt. Und noch einer!",
        ),
        (
            "es",
            "Esta es una oración en español. Otra sigue. ¡Y otra más!",
        ),
        (
            "ja",
            "これは日本語の文です。もう一つあります。そしてもう一つ。",
        ),
        (
            "ar",
            "هذه جملة باللغة العربية. تتبعها جملة أخرى. وجملة أخرى!",
        ),
    ];

    for (lang, text) in languages.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(lang), text, |b, text| {
            b.iter(|| segment(black_box(lang), black_box(text)))
        });
    }

    group.finish();
}

fn bench_paragraph_heavy_text(c: &mut Criterion) {
    let text = "First paragraph sentence one. First paragraph sentence two.\n\n\
                Second paragraph sentence one. Second paragraph sentence two.\n\n\
                Third paragraph sentence one. Third paragraph sentence two.\n\n\
                Fourth paragraph sentence one. Fourth paragraph sentence two.";

    c.bench_function("paragraph_heavy_text", |b| {
        b.iter(|| segment(black_box("en"), black_box(text)))
    });
}

fn bench_abbreviation_heavy_text(c: &mut Criterion) {
    let text = "Dr. Smith met with Prof. Johnson at the U.S. embassy. \
                They discussed the Ph.D. program at MIT. \
                The meeting was at 3 p.m. in Jan. and lasted until 5 p.m. \
                Mr. Brown and Mrs. Davis were also there.";

    c.bench_function("abbreviation_heavy_text", |b| {
        b.iter(|| segment(black_box("en"), black_box(text)))
    });
}

fn bench_quoted_text(c: &mut Criterion) {
    let text = r#"She said, "This is the first sentence." He replied, "This is the second one." 
                  "What about this?" she asked. "And this!" he exclaimed."#;

    c.bench_function("quoted_text", |b| {
        b.iter(|| segment(black_box("en"), black_box(text)))
    });
}

fn bench_real_wikipedia_sample(c: &mut Criterion) {
    // Sample from a real Wikipedia article
    let text = "The James Webb Space Telescope (JWST) is a space telescope specifically designed \
                to conduct infrared astronomy. The U.S. National Aeronautics and Space Administration \
                (NASA) led Webb's design and development and partnered with two main agencies: the \
                European Space Agency (ESA) and the Canadian Space Agency (CSA). The NASA Goddard \
                Space Flight Center (GSFC) in Maryland managed telescope development, while the Space \
                Telescope Science Institute in Baltimore on the Homewood Campus of Johns Hopkins \
                University operates Webb. The primary contractor for the project was Northrop Grumman. \
                The telescope is named after James E. Webb, who was the administrator of NASA from 1961 \
                to 1968 during the Mercury, Gemini, and Apollo programs.";

    c.bench_function("wikipedia_sample", |b| {
        b.iter(|| segment(black_box("en"), black_box(text)))
    });
}

criterion_group!(
    benches,
    benchmark_segment,
    bench_segment_by_size,
    bench_boundary_detection_by_size,
    bench_multi_language,
    bench_paragraph_heavy_text,
    bench_abbreviation_heavy_text,
    bench_quoted_text,
    bench_real_wikipedia_sample
);
criterion_main!(benches);
