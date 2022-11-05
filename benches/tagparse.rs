use criterion::{criterion_group, criterion_main, Criterion};
use tagsearch::utility::*;

// Tags generated randomly with python from /usr/share/dict/words
// between 3 and 10 words per tag (all heirarchical)
// MEDIUM -- 40 tags x 1000  lines
// TALL   -- 20 tags x 10000 lines
const MEDIUM_TAG_FILE: &str = include_str!("../medium-tag-file.md");
const TALL_TAG_FILE: &str = include_str!("../tall-narrow-tag-file.md");

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Get tags from short, fat file", |b| {
        b.iter(|| get_tags_from_string(MEDIUM_TAG_FILE))
    });
    c.bench_function("Get tags from tall, skinny file", |b| {
        b.iter(|| get_tags_from_string(TALL_TAG_FILE))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
