extern crate memmap;
extern crate thread_scoped;
extern crate libc;
extern crate memchr;

use std::str;

use self::memmap::{Mmap, Protection};
use self::memchr::memchr;

pub fn count_mmap_parallel_memchr(filepath: &str) -> u64 {
    let fmmap = Mmap::open_path(filepath, Protection::Read).expect("mmap err");
    let bytes: &[u8] = unsafe { fmmap.as_slice() };

    let mut handles: Vec<thread_scoped::JoinGuard<u64>> = Vec::new();

    for chunk in bytes.chunks(bytes.len() / 4) {
        unsafe {
            let t = thread_scoped::scoped(move || count_buf_lines(chunk));
            handles.push(t);
        };
    }

    let mut total_lines = 0;
    for h in handles {
        total_lines += h.join()
    }
    total_lines
}

fn count_buf_lines(buf: &[u8]) -> u64 {
    let mut lines = 0;
    let mut start = 0;
    while let Some(n) = memchr(b'\n', &buf[start..buf.len()]) {
        start = start + n + 1;
        lines += 1;
    }
    lines
}
