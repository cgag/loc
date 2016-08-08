#![feature(test)]
extern crate test;
extern crate count;

use test::Bencher;

use count::*;

#[bench]
fn test_count_c_reader(b: &mut Bencher) {
    b.iter(|| count("tests/data/plasma.c"))
}
