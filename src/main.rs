extern crate memmap;
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

    let fmmap = Mmap::open_path(filepath, Protection::Read).expect("mmap err");
    let bytes: &[u8] = unsafe { fmmap.as_slice() };

    // TODO(cgag): try MAP_POPULATE?
    // let mut bytes_ptr = &mut *bytes as *mut _ as *mut libc::c_void;
    // let ret = unsafe { madvise(bytes_ptr, bytes.len(), MADV_SEQUENTIAL) };
    // if ret != 0 {
    //     println!("error in madvise: {}", ret);
    //     exit(ret);
    // }

    let mut lines = 0;

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    for (i, chunk) in bytes.chunks(bytes.len() / 4).enumerate() {
        let t = thread::spawn(move || {
            for byte in chunk {
                if *byte == b'\n' {
                    lines += 1;
                }
            }
        });

        handles.push(t);
        println!("i: {}", i)
    }

    for h in handles {
        h.join();
    }

    println!("lines: {}", lines);
}
