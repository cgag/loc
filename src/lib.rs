pub mod lines;

#[macro_use]
extern crate nom;

extern crate regex;
extern crate memmap;
extern crate memchr;

use std::io::{BufReader, BufRead};
use std::fs::File;

use memmap::{Mmap, Protection};
use memchr::memchr;

use regex::Regex;
use nom::*;

// pub mod lang;

pub enum Lang {
    C, /* C_CPP_HEADER,
        * Rust,
        * Haskell,
        * Perl,
        * BourneShell, */
}
use self::Lang::*;

// Why is it called partialEq?
#[derive(Debug, PartialEq)]
pub struct Count {
    pub code: u32,
    pub comment: u32,
    pub blank: u32,
    pub lines: u32,
}

fn lang_from_extension(ext: &str) -> Lang {
    match ext {
        "c" => C,
        // "rs" => Rust,
        // "h" => C_CPP_HEADER,
        // "hs" => Haskell,
        // "perl" => Perl,
        // "sh" => BourneShell,
        _ => panic!("unrecognized ext"),
    }
}

// fn couter_for_lang(lang: Lang) -> Counter {
//     return match lang {
//         C => {
//             Counter {
//                 is_comment: |line: &str| true,
//                 is_code: |line: &str| true,
//             }
//         }
//     };
// }


// TODO(cgag): this is a horrible mess.
pub fn count_regex(filepath: &str) -> Count {
    let f = File::open(filepath).unwrap();
    let reader = BufReader::new(f);

    let mut lines = 0;
    let mut code = 0;
    let mut comments = 0;
    let mut blanks = 0;

    let single_comment = Regex::new(r"^\s*//").unwrap();
    let multi_comment_start_with_blank = Regex::new(r"^\s*/\*").unwrap();
    let multi_comment_start = Regex::new(r"/\*").unwrap();
    let multi_comment_end = Regex::new(r"\s*\*/").unwrap();
    let blank = Regex::new(r"^[:space:]*$").unwrap();

    let mut in_comment = false;
    for line in reader.lines() {
        let line = line.unwrap();
        lines += 1;

        if in_comment {
            comments += 1;
            if multi_comment_end.is_match(&line) {
                in_comment = false;
            }
            continue;
        }

        // TODO(cgag): try parsers, regex, operating on non-utf8, etc.
        // Only escelate to utf8 if needed?  When is it needed?
        if blank.is_match(&line) {
            blanks += 1;
        } else if single_comment.is_match(&line) {
            comments += 1;
        } else if multi_comment_start.is_match(&line) {
            if multi_comment_start_with_blank.is_match(&line) {
                comments += 1;
                if multi_comment_end.is_match(&line) {
                    continue;
                } else {
                    in_comment = true
                }
            } else {
                code += 1;
                if multi_comment_end.is_match(&line) {
                    continue;
                } else {
                    in_comment = true
                }
            }
        } else {
            code += 1;
        }
    }

    Count {
        code: code,
        comment: comments,
        blank: blanks,
        lines: lines,
    }
}

enum State {
    // In multiline comment
    // InSingleLineComment,
    // InMultiLineComment,
    InComment,
    // Haven't yet encounteered a non-whitespace char.
    LineStart,
    InCode,
}
use self::State::*;

struct AsciiLines<'a> {
    buf: &'a [u8],
    pos: usize,
}

struct Ascii<'a>(&'a [u8]);
impl<'a> Ascii<'a> {
    fn lines(&self) -> AsciiLines {
        AsciiLines {
            buf: self.0,
            pos: 0,
        }
    }
}

// Appears to work, now we just neeed
impl<'a> Iterator for AsciiLines<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        match memchr(b'\n', &self.buf[self.pos..self.buf.len()]) {
            Some(n) => {
                let start = self.pos;
                self.pos = self.pos + n + 1;
                Some(&self.buf[start..self.pos])
            }
            None => {
                if self.pos == self.buf.len() {
                    return None;
                }
                let start = self.pos;
                self.pos = self.buf.len();
                Some(&self.buf[start..self.pos])
            }
        }
    }
}

pub fn count_manual_bytes_with_iterator(filepath: &str) -> Count {
    let fmmap = Mmap::open_path(filepath, Protection::Read).expect("mmap err");
    let bytes: &[u8] = unsafe { fmmap.as_slice() };


    let mut lines = 0;
    let code = 0;
    let comments = 0;
    let blank = 0;

    for line in Ascii(bytes).lines() {
        lines += 1;
    }

    Count {
        code: code,
        comment: comments,
        blank: blank,
        lines: lines,
    }
}

pub fn count_manual_bytes_try1(filepath: &str) -> Count {
    let fmmap = Mmap::open_path(filepath, Protection::Read).expect("mmap err");
    let bytes: &[u8] = unsafe { fmmap.as_slice() };

    // TODO(cgag): not actually fure if this is how you represent this
    // let vtab = b'\x0B';
    // let ffeed = b'\x0C';

    let mut lines = 0;
    let mut code = 0;
    let mut comments = 0;
    let mut blank = 0;

    let mut state = LineStart;

    let mut prev_byte: &u8 = &0;
    for byte in bytes {
        state = match byte {
            &b' ' | &b'\t' | &b'\r' | &b'\x0B' | &b'\x0C' => {
                match state {
                    _ => state,
                }
            }
            &b'/' => {
                match state {
                    InCode => InCode,
                    LineStart => {
                        if prev_byte == &b'/' {
                            InComment
                        } else {
                            LineStart
                        }
                    }
                    InComment => {
                        if prev_byte == &b'*' {
                            comments += 1;
                            LineStart
                        } else {
                            InComment
                        }
                    }
                }
            }
            &b'*' => {
                match state {
                    LineStart => {
                        if prev_byte == &b'/' {
                            InComment
                        } else {
                            InCode
                        }
                    }
                    InCode => InCode,
                    InComment => InComment,
                }
            }
            &b'\n' => {
                lines += 1;
                match state {
                    InCode => {
                        code += 1;
                        LineStart
                    }
                    InComment => {
                        comments += 1;
                        InComment
                    }
                    LineStart => {
                        blank += 1;
                        LineStart
                    }
                }
            }
            &_ => {
                match state {
                    LineStart => InCode,
                    InCode => InCode,
                    InComment => InComment,
                }
            }
        };
        prev_byte = byte;
    }

    Count {
        code: code,
        comment: comments,
        blank: blank,
        lines: lines,
    }
}

// pub fn count_nom(filepath: &str) -> Count {
//     let fmmap = Mmap::open_path(filepath, Protection::Read).expect("mmap err");
//     let bytes: &[u8] = unsafe { fmmap.as_slice() };

//     // let p = named!(blank, many0!(is_space));
//     Count {
//         code: 0,
//         comment: 0,
//         blank: 0,
//         lines: 0,
//     }
// }
