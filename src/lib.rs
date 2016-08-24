pub mod lines;

#[macro_use]
extern crate nom;

extern crate regex;
extern crate memmap;
extern crate memchr;
extern crate ascii;

use std::path::Path;
use std::cmp;
use std::fmt;

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

pub struct LangTotal {
    pub files: u32,
    pub count: Count,
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
    Make,
    INI,
    Assembly,
    Yacc,
    Awk,
    XML,
    Unrecognized,
}

impl Lang {
    pub fn to_s(&self) -> &str {
        match *self {
            Lang::C => "C",
            Lang::CCppHeader => "C/C++ Header",
            Lang::Rust => "Rust",
            Lang::Ruby => "Ruby",
            Lang::Haskell => "Haskell",
            Lang::Perl => "Perl",
            Lang::BourneShell => "Bourne Shell",
            Lang::Make => "Make",
            Lang::INI => "INI",
            Lang::Assembly => "Assembly",
            Lang::Yacc => "Yacc",
            Lang::Awk => "Awk",
            Lang::XML => "XMl",
            Lang::Unrecognized => "Unrecognized",
        }
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(self.to_s())
    }
}

pub fn lang_from_ext(filepath: &str) -> Lang {
    let path = Path::new(filepath);
    let ext = match path.extension() {
        Some(os_str) => os_str.to_str().unwrap().to_lowercase(),
        None => path.file_name().unwrap().to_str().unwrap().to_lowercase(),
    };

    match &*ext {
        "c" => Lang::C,
        "h" | "hh" | "hpp" | "hxx" => Lang::CCppHeader,
        "rs" => Lang::Rust,
        "hs" => Lang::Haskell,
        "pl" => Lang::Perl,
        "rb" => Lang::Ruby,
        "makefile" | "mk" => Lang::Make,
        "ini" => Lang::INI,
        "s" | "asm" => Lang::Assembly,
        "y" => Lang::Yacc,
        "awk" => Lang::Awk,
        "xml" => Lang::XML,

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
        Lang::INI => CT::Single(";"),
        // TODO(cgag): Well, some architectures use ;, @, |, etc.
        // Need a way to specify more than one possible comment char.
        Lang::Assembly => CT::Multi("#", "/*", "*/"),
        // TODO(cgag): Welp, single is not always necessary
        Lang::XML => CT::Multi("GIEBBBLLLlXKJJKJK", "<!--", "-->"),
        Lang::BourneShell | Lang::Ruby | Lang::Make | Lang::Awk => sh_style,
        // TODO(cgag): not 100% that yacc belongs here.
        Lang::C | Lang::CCppHeader | Lang::Rust | Lang::Yacc => c_style,
        // Default to C style
        Lang::Unrecognized => c_style,
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
    let fmmap = match Mmap::open_path(filepath, Protection::Read) {
        Ok(mmap) => mmap,
        Err(e) => {
            println!("mmap err for {}: {}", filepath, e);
            return Count::default();
        }
    };
    let bytes: &[u8] = unsafe { fmmap.as_slice() };

    let mut lines = 0;
    let mut code = 0;
    let mut comments = 0;
    let mut blanks = 0;

    let a = Ascii(bytes);
    for byte_line in a.lines() {
        let line = match std::str::from_utf8(byte_line) {
            Ok(s) => s,
            Err(_) => return Count::default(),
        };
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

    let fmmap = match Mmap::open_path(filepath, Protection::Read) {
        Ok(mmap) => mmap,
        Err(e) => {
            println!("mmap err for {}: {}", filepath, e);
            return Count::default();
        }
    };
    let bytes: &[u8] = unsafe { fmmap.as_slice() };

    let mut lines = 0;
    let mut code = 0;
    let mut comments = 0;
    let mut blanks = 0;

    let mut in_comment = false;

    let a = Ascii(bytes);
    for byte_line in a.lines() {
        let line = match std::str::from_utf8(byte_line) {
            Ok(s) => s,
            Err(_) => return Count::default(),
        };
        lines += 1;

        let trimmed = line.trim_left();
        if trimmed.is_empty() {
            blanks += 1;
            continue;
        };

        if !in_comment && trimmed.starts_with(single_line_start) {
            comments += 1;
            continue;
        }

        if !(trimmed.contains(multiline_start) || trimmed.contains(multiline_end)) {
            if in_comment {
                comments += 1;
            } else {
                code += 1;
            }
            continue;
        }


        let start_len = multiline_start.len();
        let end_len = multiline_end.len();

        let mut pos = 0;
        let mut found_code = false;
        'outer: while pos < trimmed.len() {
            // TODO(cgag): must be a less stupid way to do this
            for i in pos..(pos + cmp::max(start_len, end_len) + 1) {
                if !trimmed.is_char_boundary(i) {
                    pos += 1;
                    continue 'outer;
                }
            }

            if pos + start_len <= trimmed.len() &&
               &trimmed[pos..pos + start_len] == multiline_start {
                pos += start_len;
                in_comment = true;
            } else if pos + end_len <= trimmed.len() && &trimmed[pos..pos + end_len] == multiline_end {
                pos += end_len;
                in_comment = false;
            } else if !in_comment {
                found_code = true;
                pos += 1;
            } else {
                pos += 1;
            }
        }

        if found_code {
            code += 1;
        } else {
            comments += 1;
        }
    }

    Count {
        code: code,
        comment: comments,
        blank: blanks,
        lines: lines,
    }
}
