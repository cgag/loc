#![feature(test)]
extern crate test;
extern crate count;

use test::Bencher;

use count::lines::*;

// #[bench]
// fn bench_count_mmap_parallel(b: &mut Bencher) {
//     b.iter(|| count_mmap_parallel("small"))
// }

// #[bench]
// fn bench_count_bufread_serial(b: &mut Bencher) {
//     b.iter(|| count_bufread_serial("small"))
// }

// #[bench]
// fn bench_count_bufread_serial_fadvise(b: &mut Bencher) {
//     b.iter(|| count_bufread_serial_fadvise("small"))
// }

// #[bench]
// fn bench_count_manual_read_memchr_fadvise(b: &mut Bencher) {
//     b.iter(|| count_manual_read_memchr_fadvise("small"))
// }

// #[bench]
// fn bench_count_mmap_serial(b: &mut Bencher) {
//     b.iter(|| count_mmap_serial("small"))
// }

// #[bench]
// fn bench_count_mmap_serial_memchr(b: &mut Bencher) {
//     b.iter(|| count_mmap_serial_memchr("small"))
// }

// #[bench]
// fn bench_count_mmap_serial_madvise(b: &mut Bencher) {
//     b.iter(|| count_mmap_serial_madvise("small"))
// }

// #[bench]
// fn bench_count_mmap_serial_madvise_memchr(b: &mut Bencher) {
//     b.iter(|| count_mmap_serial_madvise_memchr("small"))
// }

// #[bench]
// fn bench_count_mmap_parallel_memchr(b: &mut Bencher) {
//     b.iter(|| count_mmap_parallel_memchr("small"))
// }
