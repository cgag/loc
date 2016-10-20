#![feature(test)]
extern crate test;
extern crate loc;

use test::Bencher;

use loc::*;

#[bench]
fn test_count_c_reader(b: &mut Bencher) {
    b.iter(|| count("tests/data/plasma.c"))
}
