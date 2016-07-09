extern crate memmap;
extern crate thread_scoped;
extern crate libc;
extern crate memchr;

use std::env;
use std::thread;
use std::io::{BufReader, BufRead, Read};
use std::fs::File;
use std::process::exit;
use memmap::{Mmap, Protection};
use std::str;
use libc::{madvise, POSIX_MADV_SEQUENTIAL, MADV_SEQUENTIAL, MADV_WILLNEED, posix_fadvise,
           POSIX_FADV_SEQUENTIAL};
use std::os::unix::io::AsRawFd;
use memchr::memchr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough args!");
        exit(1);
    }
    let filepath = &args[1];
    // let total_lines = count_bufread_serial(filepath);
    // let total_lines = count_bufread_serial_fadvise(filepath);
    // let total_lines = count_manual_read_fadvise(filepath);
    // let total_lines = count_manual_read_parallel_fadvise_memchr(filepath);
    // let total_lines = count_manual_read_memchr_fadvise(filepath);
    // let total_lines = count_mmap_serial(filepath);
    // let total_lines = count_mmap_serial_memchr(filepath);
    // let total_lines = count_mmap_serial_madvise_memchr(filepath);
    // let total_lines = count_mmap_parallel(filepath);
    // let total_lines = count_mmap_parallel_memchr(filepath);
    let total_lines = count_mmap_parallel_memchr_populate(filepath);
    println!("lines: {}", total_lines);
}


// NOTE(cgag): Super slow!  Why?  Takes 6 seconds.  More than twice as slow as
// serial mmap.
fn count_bufread_serial(filepath: &str) -> u64 {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    reader.lines().fold(0, |acc, _| acc + 1)
}

fn count_bufread_serial_fadvise(filepath: &str) -> u64 {
    let file = File::open(filepath).unwrap();
    let file_size: i64 = file.metadata().unwrap().len() as i64;
    let ret = unsafe { posix_fadvise(file.as_raw_fd(), 0, file_size, POSIX_FADV_SEQUENTIAL) };
    if ret != 0 {
        println!("error in fadvise: {}", ret);
        exit(ret);
    }
    let reader = BufReader::new(file);

    reader.lines().fold(0, |acc, _| acc + 1)
}

fn count_manual_read_fadvise(filepath: &str) -> u64 {
    let mut file = File::open(filepath).unwrap();
    let file_size: i64 = file.metadata().unwrap().len() as i64;
    // let ret = unsafe { posix_fadvise(file.as_raw_fd(), 0, file_size, POSIX_FADV_SEQUENTIAL) };
    let ret = unsafe { posix_fadvise(file.as_raw_fd(), 0, 0, POSIX_FADV_SEQUENTIAL) };
    if ret != 0 {
        println!("error in fadvise: {}", ret);
        exit(ret);
    }

    // TODO(cgag): what's a page size? how many pages should this be?  Should
    // it be the size of L3 cache, L2, etc?
    // Using 4096 as oit's listed as the cache size in /proc/cpuinfo
    let mut buf: [u8; 1024 * 16] = [0; 1024 * 16];
    let mut lines = 0;
    loop {
        match file.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                for i in 0..n {
                    if buf[i] == b'\n' {
                        lines += 1;
                    }
                }
            }
            _ => panic!("shit"),
        }
    }
    lines
}


// TODO(cgag): read more about how memchr is implemented
fn count_manual_read_memchr_fadvise(filepath: &str) -> u64 {
    let mut file = File::open(filepath).unwrap();
    let file_size: i64 = file.metadata().unwrap().len() as i64;
    // let ret = unsafe { posix_fadvise(file.as_raw_fd(), 0, file_size, POSIX_FADV_SEQUENTIAL) };
    let ret = unsafe { posix_fadvise(file.as_raw_fd(), 0, 0, POSIX_FADV_SEQUENTIAL) };
    if ret != 0 {
        println!("error in fadvise: {}", ret);
        exit(ret);
    }

    // TODO(cgag): what's a page size? how many pages should this be?  Should
    // it be the size of L3 cache, L2, etc?
    // Using 4096 as oit's listed as the cache size in /proc/cpuinfo
    let mut buf: [u8; 1024 * 16] = [0; 1024 * 16];
    let mut lines = 0;
    loop {
        match file.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => lines += count_buf_lines(&buf[0..n]),
            _ => panic!("shit"),
        }
    }
    lines
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

fn count_mmap_serial_memchr(filepath: &str) -> u64 {
    let fmmap = Mmap::open_path(filepath, Protection::Read).expect("mmap err");
    let bytes: &[u8] = unsafe { fmmap.as_slice() };
    count_buf_lines(bytes)
}


fn count_mmap_serial_madvise(filepath: &str) -> u64 {
    let mut fmmap = Mmap::open_path(filepath, Protection::Read).expect("mmap err");
    let mut bytes: &mut [u8] = unsafe { fmmap.as_mut_slice() };

    // TODO(cgag): try MAP_POPULATE?
    let mut bytes_ptr = &mut *bytes as *mut _ as *mut libc::c_void;
    let ret = unsafe { madvise(bytes_ptr, bytes.len(), POSIX_MADV_SEQUENTIAL) };
    // let ret = unsafe { madvise(bytes_ptr, bytes.len(), MADV_SEQUENTIAL) };
    if ret != 0 {
        println!("error in madvise: {}", ret);
        exit(ret);
    }

    let mut lines = 0;
    for byte in bytes {
        if *byte == b'\n' {
            lines += 1;
        }
    }
    lines
}

fn count_mmap_serial_madvise_memchr(filepath: &str) -> u64 {
    let mut fmmap = Mmap::open_path(filepath, Protection::Read).expect("mmap err");
    let mut bytes: &mut [u8] = unsafe { fmmap.as_mut_slice() };

    // TODO(cgag): try MAP_POPULATE?
    let mut bytes_ptr = &mut *bytes as *mut _ as *mut libc::c_void;
    let ret = unsafe { madvise(bytes_ptr, bytes.len(), POSIX_MADV_SEQUENTIAL) };
    // let ret = unsafe { madvise(bytes_ptr, bytes.len(), MADV_SEQUENTIAL) };
    if ret != 0 {
        println!("error in madvise: {}", ret);
        exit(ret);
    }

    count_buf_lines(bytes)
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

fn count_mmap_parallel_memchr(filepath: &str) -> u64 {
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


fn count_mmap_parallel_memchr_populate(filepath: &str) -> u64 {
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
    loop {
        match memchr(b'\n', &buf[start..buf.len()]) {
            Some(nn) => {
                start = start + nn + 1;
                lines += 1;
            }
            None => {
                break;
            }
        }
    }
    lines
}
