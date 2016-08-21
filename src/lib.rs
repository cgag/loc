pub mod lines;

#[macro_use]
extern crate nom;

extern crate regex;
extern crate memmap;
extern crate memchr;
extern crate ascii;

use std::path::Path;

use memmap::{Mmap, Protection};
use memchr::memchr;

// Why is it called partialEq?
#[derive(Debug, PartialEq, Default, Clone)]
pub struct Count {
    pub code: u32,
    pub comment: u32,
    pub blank: u32,
    pub lines: u32,
}

impl Count {
    pub fn merge(&mut self, o: &Count) {
        self.code += o.code;
        self.comment += o.comment;
        self.blank += o.blank;
        self.lines += o.lines;
    }
}

pub enum LineConfig<'a> {
    Multi {
        single_start: &'a str,
        multi_start: &'a str,
        multi_end: &'a str,
    },
    Single {
        single_start: &'a str,
    },
}

// Do any languages actually use utf8 chars as comment chars?
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Lang {
    C,
    CCppHeader,
    Rust,
    Ruby,
    Haskell,
    Perl,
    BourneShell,
    Unrecognized,
}

pub fn lang_from_ext(filepath: &str) -> Lang {
    let ext = match Path::new(filepath).extension() {
        Some(os_str) => os_str.to_str().unwrap().to_lowercase(),
        None => String::from(""),
    };

    match &*ext {
        "c" => Lang::C,
        "h" => Lang::CCppHeader,
        "rs" => Lang::Rust,
        "hs" => Lang::Haskell,
        "pl" => Lang::Perl,
        "rb" => Lang::Ruby,
        // TODO(cgag): What's the correct extension? Any? Pragma?
        "sh" => Lang::BourneShell,
        // Probably dumb to just default to C.
        _ => Lang::Unrecognized,
    }
}

pub fn counter_config_for_lang<'a>(lang: &Lang) -> LineConfig<'a> {
    enum CT<'a> {
        Single(&'a str),
        Multi(&'a str, &'a str, &'a str),
    }
    // use self::ConfigTuple::*;

    let c_style = CT::Multi("//", "/*", "*/");
    let sh_style = CT::Single("#");

    let ctuple = match *lang {
        Lang::Haskell => CT::Multi("--", "{-", "-}"),
        Lang::Perl => CT::Multi("#", "=pod", "=cut"),
        Lang::BourneShell | Lang::Ruby => sh_style,
        // Lang::C | Lang::CCppHeader | Lang::Rust => c_style,
        // Default to C style
        _ => c_style,
    };

    match ctuple {
        CT::Multi(single, start, end) => {
            LineConfig::Multi {
                single_start: single,
                multi_start: start,
                multi_end: end,
            }
        }
        CT::Single(single) => LineConfig::Single { single_start: single },
    }
}

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
                // - 1 to drop \n char
                Some(&self.buf[start..(self.pos - 1)])
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

pub fn count(filepath: &str) -> Count {
    let lang = lang_from_ext(filepath);
    let config = counter_config_for_lang(&lang);
    match config {
        LineConfig::Single { single_start: single } => count_mmap_unsafe_single(filepath, single),
        LineConfig::Multi { single_start: single, multi_start, multi_end } => {
            count_mmap_unsafe_multi(filepath, single, multi_start, multi_end)
        }
    }
}

pub fn count_mmap_unsafe_single(filepath: &str, single_start: &str) -> Count {
    let fmmap = Mmap::open_path(filepath, Protection::Read).expect("mmap err");
    let bytes: &[u8] = unsafe { fmmap.as_slice() };

    let mut lines = 0;
    let mut code = 0;
    let mut comments = 0;
    let mut blanks = 0;

    let a = Ascii(bytes);
    for byte_line in a.lines() {
        let line = unsafe { std::str::from_utf8_unchecked(byte_line) };
        lines += 1;

        let trimmed = line.trim_left();
        if trimmed.is_empty() {
            blanks += 1;
        } else if trimmed.starts_with(single_start) {
            comments += 1;
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

pub fn count_mmap_unsafe_multi(filepath: &str,
                               single_start: &str,
                               multi_start: &str,
                               multi_end: &str)
                               -> Count {

    let single_line_start = single_start;
    let multiline_start = multi_start;
    let multiline_end = multi_end;

    let fmmap = Mmap::open_path(filepath, Protection::Read).expect("mmap err");
    let bytes: &[u8] = unsafe { fmmap.as_slice() };

    let mut lines = 0;
    let mut code = 0;
    let mut comments = 0;
    let mut blanks = 0;

    let mut in_comment = false;

    let a = Ascii(bytes);
    for byte_line in a.lines() {
        println!("reading line");
        let line = unsafe { std::str::from_utf8_unchecked(byte_line) };
        println!("read line");
        lines += 1;

        let trimmed = line.trim_left();
        if trimmed.is_empty() {
            // println!("Blank: {}", line);
            blanks += 1;
            continue;
        };

        // if in_comment {
        //     println!("Comment: {}", line);
        //     if trimmed.contains(multi_end) {
        //         // tmp
        //         in_comment = false;
        //     } else {
        //         comments += 1;
        //     }
        // } else {

        if !in_comment {
            if trimmed.starts_with(single_line_start) {
                println!("Comment: {}", line);
                comments += 1;
                continue;
            }
        }

        let mut pos = 0;
        let mut found_code = false;

        let start_len = multiline_start.len();
        let end_len = multiline_end.len();

        // TODO(cgag): Skip this loop if we don't contain start or end?
        // Should be faster.  Test it.
        while pos < trimmed.len() {
            if pos + start_len <= trimmed.len() {
                // println!("start: {}", &trimmed[pos..pos + start_len])
            }

            if pos + end_len <= trimmed.len() {
                // println!("end: {}", &trimmed[pos..pos + end_len])
            }

            if pos + start_len <= trimmed.len() &&
               &trimmed[pos..pos + start_len] == multiline_start {
                // println!("Found start {}", pos);
                pos += start_len;
                in_comment = true;
            } else if pos + end_len <= trimmed.len() && &trimmed[pos..pos + end_len] == multiline_end {
                // println!("Found end: {}", trimmed);
                pos += end_len;
                in_comment = false;
            } else if !in_comment {
                // println!("Found code: {}", trimmed);
                found_code = true;
                pos += 1;
            } else {
                pos += 1;
            }
        }

        if found_code {
            // println!("Code: {}", trimmed);
            code += 1;
        } else {
            // println!("Comment: {}", trimmed);
            comments += 1;
        }
        // }
    }

    // TODO(cgag): try parsers, regex, operating on non-utf8, etc.
    // Only escelate to utf8 if needed?  When is it needed?
    Count {
        code: code,
        comment: comments,
        blank: blanks,
        lines: lines,
    }
}
