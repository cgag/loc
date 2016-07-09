extern crate memmap;
extern crate thread_scoped;
extern crate libc;

use std::env;
use std::thread;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::process::exit;
use memmap::{Mmap, Protection};
use std::str;
use libc::{madvise, MADV_SEQUENTIAL, MADV_WILLNEED};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough args!");
        exit(1);
    }
    let filepath = &args[1];
    let total_lines = count_mmap_serial(filepath);
    // let total_lines = count_mmap_parallel(filepath);
    println!("lines: {}", total_lines);
}

fn count_mmap_serial(filepath: &str) -> u64 {
    let fmmap = Mmap::open_path(filepath, Protection::Read).expect("mmap err");
    let bytes: &[u8] = unsafe { fmmap.as_slice() };

    let mut lines = 0;
    for byte in bytes {
        if *byte == b'\n' {
            lines += 1;
        }
    }
    lines
}

fn count_mmap_parallel(filepath: &str) -> u64 {
    let fmmap = Mmap::open_path(filepath, Protection::Read).expect("mmap err");
    let bytes: &[u8] = unsafe { fmmap.as_slice() };

    let mut handles: Vec<thread_scoped::JoinGuard<u64>> = Vec::new();

    for chunk in bytes.chunks(bytes.len() / 4) {
        unsafe {
            let t = thread_scoped::scoped(move || {
                let mut lines = 0;
                for byte in chunk {
                    if *byte == b'\n' {
                        lines += 1;
                    }
                }
                lines
            });
            handles.push(t);
        };
    }

    let mut total_lines = 0;
    for h in handles {
        total_lines += h.join()
    }
    total_lines
}
