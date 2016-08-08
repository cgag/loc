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
    let ctuple = match *lang {
        Lang::Haskell => CT::Multi("--", "{-", "-}"),
        Lang::Perl => CT::Multi("#", "=pod", "=cut"),
        Lang::BourneShell => CT::Single("#"),
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
        if trimmed == "" {
            blanks += 1;
            continue;
        } else if trimmed.starts_with(single_start) {
            comments += 1;
            continue;
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
        let line = unsafe { std::str::from_utf8_unchecked(byte_line) };
        lines += 1;

        let trimmed = line.trim_left();
        if trimmed == "" {
            println!("Blank: {}", line);
            blanks += 1;
            continue;
        };

        if in_comment {
            println!("Comment: {}", line);
            comments += 1;
            if trimmed.contains(multiline_end) {
                in_comment = false;
            }
        } else {
            if trimmed.starts_with(single_line_start) {
                println!("Comment: {}", line);
                comments += 1;
                continue;
            }

            // This code really sucks. Handles `/* lol */ func()` correctly as code, but
            // it's not recursive in any way.  It would count /* x */ /* x */ func() as a
            // comment.
            if trimmed.starts_with(multiline_start) {
                if trimmed.contains(multiline_end) {
                    let rest = &trimmed[multiline_start.len()..];
                    if let Some(end_pos) = rest.find(multiline_end) {
                        let after_comment = &rest[end_pos + multiline_end.len()..];
                        let after_trimmed = after_comment.trim_left();
                        if after_trimmed.is_empty() {
                            println!("Comment: {}", line);
                            comments += 1;
                        } else if after_trimmed.starts_with(multiline_start) ||
                           after_trimmed.starts_with(single_start) {
                            println!("Comment: {}", line);
                            comments += 1;
                        } else {
                            println!("Code: {}", line);
                            code += 1;
                        }
                    }
                } else {
                    println!("Comment: {}", line);
                    comments += 1;
                    in_comment = true;
                }
            } else {
                match trimmed.find(multiline_start) {
                    Some(pos) => {
                        println!("Code: {}", line);
                        code += 1;
                        // TODO(cgag): What the hell is +2.  len of of what?
                        if trimmed[pos + multiline_start.len()..].contains(multiline_end) {
                            continue;
                        }
                        in_comment = true;
                    }
                    None => {
                        println!("Code: {}", line);
                        code += 1;
                    }
                }
            }
        }
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
