#![feature(test)]
extern crate test;
extern crate count;

use test::Bencher;

use count::*;

#[bench]
fn test_count_c_regex(b: &mut Bencher) {
    b.iter(|| count_regex("tests/data/plasma.c"))
}

#[bench]
fn test_count_c_mmap_safe(b: &mut Bencher) {
    b.iter(|| count_mmap_safe("tests/data/plasma.c"))
}

#[bench]
fn test_count_c_mmap_unsafe(b: &mut Bencher) {
    b.iter(|| count_mmap_unsafe("tests/data/plasma.c"))
}

#[bench]
fn test_count_c_reader(b: &mut Bencher) {
    b.iter(|| count_reader("tests/data/plasma.c"))
}

#[bench]
fn test_count_c_reader2(b: &mut Bencher) {
    b.iter(|| count_reader2("tests/data/plasma.c"))
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
