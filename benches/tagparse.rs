#[macro_use]
extern crate bencher;

use bencher::Bencher;
use tagsearch::utility::*;

// Tags generated randomly with python from /usr/share/dict/words
// between 3 and 10 words per tag (all heirarchical)
// MEDIUM -- 40 tags x 1000  lines
// TALL   -- 20 tags x 10000 lines
const MEDIUM_TAG_FILE: &str = include_str!("../medium-tag-file.md");
const TALL_TAG_FILE: &str = include_str!("../tall-narrow-tag-file.md");

fn bench_get_tags_medium(b: &mut Bencher) {
    b.iter(|| get_tags_from_string(MEDIUM_TAG_FILE));
}

fn bench_get_tags_tall(b: &mut Bencher) {
    b.iter(|| get_tags_from_string(TALL_TAG_FILE));
}

fn bench_get_tags_2_medium(b: &mut Bencher) {
    b.iter(|| get_tags_from_string_2(MEDIUM_TAG_FILE));
}

fn bench_get_tags_2_tall(b: &mut Bencher) {
    b.iter(|| get_tags_from_string_2(TALL_TAG_FILE));
}

benchmark_group!(
    benches,
    bench_get_tags_medium,
    bench_get_tags_2_medium,
    bench_get_tags_tall,
    bench_get_tags_2_tall
);

benchmark_main!(benches);
