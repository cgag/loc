#![feature(test)]
extern crate test;
extern crate count;

use test::Bencher;

use count::{Count, count_regex, count_manual_bytes_with_iterator, count_manual_bytes_try1};

#[bench]
fn test_count_c_regex(b: &mut Bencher) {
    b.iter(|| count_regex("tests/data/plasma.c"))
}

// #[bench]
// fn test_count_c_nom(b: &mut Bencher) {
//     b.iter(|| count_nom("tests/data/plasma.c"))
// }

#[bench]
fn test_count_c_manual_bytes_try1(b: &mut Bencher) {
    b.iter(|| count_manual_bytes_try1("tests/data/plasma.c"))
}

#[bench]
fn test_count_c_manual_bytes(b: &mut Bencher) {
    b.iter(|| count_manual_bytes_with_iterator("tests/data/plasma.c"))
}
