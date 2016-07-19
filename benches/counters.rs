#![feature(test)]
extern crate test;
extern crate count;

use test::Bencher;

use count::{Count, count_regex, count_nom, count_manual_bytes};

#[bench]
fn test_count_c_regex(b: &mut Bencher) {
    b.iter(|| count_regex("tests/data/plasma.c"))
}

#[bench]
fn test_count_c_nom(b: &mut Bencher) {
    b.iter(|| count_nom("tests/data/plasma.c"))
}

#[bench]
fn test_count_c_manual_bytes(b: &mut Bencher) {
    b.iter(|| count_manual_bytes("tests/data/plasma.c"))
}
