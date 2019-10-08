#![feature(test)]
extern crate loc;
extern crate test;

use test::Bencher;

use loc::*;

#[bench]
fn test_count_c(b: &mut Bencher) {
    b.iter(|| count("tests/data/plasma.c"))
}

#[bench]
fn test_count_lua(b: &mut Bencher) {
    b.iter(|| count("tests/data/lua-big.lua"))
}
