pub mod lines;

#[macro_use]
extern crate nom;

extern crate regex;
extern crate memmap;
extern crate memchr;
extern crate ascii;

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

use memmap::{Mmap, Protection};
use memchr::memchr;

use regex::Regex;

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

pub enum CounterConfig<'a> {
    MultiLine {
        single_start: &'a str,
        multi_start: &'a str,
        multi_end: &'a str,
    },
    SingleLine {
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

        // Probably dumb to just default to C.
        _ => Lang::Unrecognized,
    }
}

pub fn counter_config_for_lang<'a>(lang: &Lang) -> CounterConfig<'a> {
    enum CT<'a> {
        Single(&'a str),
        Multi(&'a str, &'a str, &'a str),
    }
    // use self::ConfigTuple::*;

    let c_style = CT::Multi("//", "/*", "*/");
    let ctuple = match *lang {
        Lang::C => c_style,
        Lang::CCppHeader => c_style,
        Lang::Rust => c_style,
        Lang::Haskell => CT::Multi("--", "{-", "-}"),
        Lang::Perl => CT::Multi("#", "=pod", "=cut"),
        // TODO(cgag) need to handle langs without multiline comments
        Lang::BourneShell => CT::Single("#"),
        // Default to C style
        _ => c_style,
    };

    match ctuple {
        CT::Multi(single, start, end) => {
            CounterConfig::MultiLine {
                single_start: single,
                multi_start: start,
                multi_end: end,
            }
        }
        CT::Single(single) => CounterConfig::SingleLine { single_start: single },
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

pub fn count(filepath: &str, config: CounterConfig) -> Count {
    match config {
        CounterConfig::SingleLineCommentConfig { single_start: single_start } => {
            count_mmap_unsafe_single(filepath, single_start)
        }
        CounterConfig::MultiLineConfig { single_start: single,
                                         multi_start: multi_start,
                                         multi_end: multi_end } => {
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
    let multiline_start_char = multi_start;
    let multiline_end_char = multi_end;

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
            blanks += 1;
            continue;
        };

        if in_comment {
            comments += 1;
            if trimmed.contains(multiline_end_char) {
                in_comment = false;
            }
        } else {
            if trimmed.starts_with(single_line_start) {
                comments += 1;
                continue;
            }

            if trimmed.starts_with(multiline_start_char) {
                comments += 1;
                if trimmed.contains(multiline_end_char) {
                    continue;
                }
                in_comment = true;
            } else {
                match trimmed.find(multiline_start_char) {
                    Some(pos) => {
                        code += 1;
                        if trimmed[pos + 2..].contains(multiline_end_char) {
                            continue;
                        }
                        in_comment = true;
                    }
                    None => {
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

pub fn count_mmap_safe(filepath: &str) -> Count {
    let single_line_start = "//";
    let multiline_start_char = "/*";
    let multiline_end_char = "*/";

    let fmmap = Mmap::open_path(filepath, Protection::Read).expect("mmap err");
    let bytes: &[u8] = unsafe { fmmap.as_slice() };

    let mut lines = 0;
    let mut code = 0;
    let mut comments = 0;
    let mut blanks = 0;

    let mut in_comment = false;

    let a = Ascii(bytes);
    for byte_line in a.lines() {
        let line = std::str::from_utf8(byte_line).expect("err parsing bytes");
        lines += 1;

        let trimmed = line.trim_left();
        if trimmed == "" {
            blanks += 1;
            continue;
        };

        if in_comment {
            comments += 1;
            if trimmed.contains(multiline_end_char) {
                in_comment = false;
            }
        } else {
            if trimmed.starts_with(single_line_start) {
                comments += 1;
                continue;
            }

            if trimmed.starts_with(multiline_start_char) {
                comments += 1;
                if trimmed.contains(multiline_end_char) {
                    continue;
                }
                in_comment = true;
            } else {
                match trimmed.find(multiline_start_char) {
                    Some(pos) => {
                        code += 1;
                        if trimmed[pos + 2..].contains(multiline_end_char) {
                            continue;
                        }
                        in_comment = true;
                    }
                    None => {
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

pub fn count_reader2(filepath: &str) -> Count {
    let single_line_start = "//";
    let multiline_start_char = "/*";
    let multiline_end_char = "*/";

    let f = File::open(filepath).unwrap();
    let reader = BufReader::new(f);

    let mut lines = 0;
    let mut code = 0;
    let mut comments = 0;
    let mut blanks = 0;

    let mut in_comment = false;

    for line in reader.lines() {
        let line = line.unwrap();
        lines += 1;

        let trimmed = line.trim_left();
        if trimmed == "" {
            blanks += 1;
            continue;
        };

        if in_comment {
            comments += 1;
            if trimmed.contains(multiline_end_char) {
                in_comment = false;
            }
        } else {
            if trimmed.starts_with(single_line_start) {
                comments += 1;
                continue;
            }

            if trimmed.starts_with(multiline_start_char) {
                comments += 1;
                if trimmed.contains(multiline_end_char) {
                    continue;
                }
                in_comment = true;
            } else {
                match trimmed.find(multiline_start_char) {
                    Some(pos) => {
                        code += 1;
                        if trimmed[pos + 2..].contains(multiline_end_char) {
                            continue;
                        }
                        in_comment = true;
                    }
                    None => {
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


#[derive(Debug, PartialEq)]
enum State2 {
    Comment,
    Code,
    Blank,
}
use self::State2::*;

pub fn count_manual_bytes_with_iterator(filepath: &str) -> Count {
    let fmmap = Mmap::open_path(filepath, Protection::Read).expect("mmap err");
    let bytes: &[u8] = unsafe { fmmap.as_slice() };

    let mut lines = 0;
    let mut code = 0;
    let mut comments = 0;
    let mut blank = 0;

    let mut in_comment = false;
    let mut state = Blank;
    let a = Ascii(bytes);
    let mut prev_byte = b'\0';
    for byte_line in a.lines() {

        if in_comment {
            comments += 1;
            let mut c_prev_byte = 0;
            for byte in byte_line {
                // would need to mathc bytes in here
                // if state == Blank {
                //     println!("State: {:?}, line: {}",
                //              state,
                //              unsafe { std::str::from_utf8_unchecked(byte_line) });
                // }
                if c_prev_byte == b'*' && *byte == b'/' {
                    in_comment = false;
                }
                c_prev_byte = *byte;
            }
        } else {
            for byte in byte_line {
                match *byte {
                    b'/' => {
                        match prev_byte {
                            b'*' => {
                                in_comment = false;
                            }
                            b'/' => {
                                state = Comment;
                                break;
                            }
                            _ => {}
                        }
                    }
                    b'*' => {
                        if prev_byte == b'/' {
                            // dumb hack to prevent /*/ from starting and ending a comment
                            // at once.  Better way?
                            // Why clippy claiming this is never read? it must be, right?
                            prev_byte = 0;
                            in_comment = true;
                            state = Comment;
                        }
                    }
                    b' ' | b'\t' | b'\r' | b'\x0B' | b'\x0C' => {}
                    _ => {
                        if !in_comment {
                            state = Code;
                            break;
                        }
                    }
                }
                prev_byte = *byte;
            }
        }

        // if state == Blank {
        //     println!("State: {:?}, line: {}",
        //              state,
        //              unsafe { std::str::from_utf8_unchecked(byte_line) });
        // }

        lines += 1;

        match state {
            Blank => {
                blank += 1;
            }
            Comment => {
                comments += 1;
            }
            Code => {
                code += 1;
            }
        }

        state = Blank;
    }

    Count {
        code: code,
        comment: comments,
        blank: blank,
        lines: lines,
    }
}

// fn bytes_contain(b &[u8], needle &str) {
//     for nbyte in str.as_bytes() {
//         match memchr(b'\n', ) {
//         }
//     }
// }

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
        state = match *byte {
            b'/' => {
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
            b'*' => {
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
            b'\n' => {
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
            _ => state,
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
        } else if line.contains("/*") {
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

pub fn count_reader(filepath: &str) -> Count {
    let f = File::open(filepath).unwrap();
    let reader = BufReader::new(f);

    let mut lines = 0;
    let mut code = 0;
    let mut comments = 0;
    let mut blanks = 0;

    let mut in_comment = false;
    for line in reader.lines() {
        let line = line.unwrap();
        lines += 1;

        let trimmed = line.trim_left();
        if trimmed == "" {
            blanks += 1;
            continue;
        };

        if in_comment {
            comments += 1;
            if trimmed.contains("*/") {
                in_comment = false;
            }
            continue;
        }

        if trimmed.starts_with("//") {
            comments += 1;
            continue;
        }

        if trimmed.starts_with("/*") {
            comments += 1;
            if trimmed.contains("*/") {
                continue;
            }

            in_comment = true;
            continue;
        }

        if trimmed.contains("/*") {
            code += 1;
            // TODO(cgag): handle /*/,
            // TODO: pos needs to be after /*
            if trimmed.contains("*/") {
                continue;
            }

            in_comment = true;
            continue;
        }

        code += 1;
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
