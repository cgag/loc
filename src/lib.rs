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
    SingleMulti {
        single_start: &'a str,
        multi_start: &'a str,
        multi_end: &'a str,
    },
    SingleOnly {
        single_start: &'a str,
    },
    MultiOnly {
        multi_start: &'a str,
        multi_end: &'a str,
    }, /* Everything {
        *     singles: Vec<&'a str>,
        *     multies: Vec<(&'a str, &'a str)>,
        * }, */
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

    // Asp => Language::new_single(vec!["'", "REM"]),
    // Autoconf => Language::new_single(vec!["#", "dnl"]),
    // Clojure => Language::new_single(vec![";","#"]),
    // FortranLegacy => Language::new_single(vec!["c","C","!","*"]),
    // FortranModern => Language::new_single(vec!["!"]),
    //
    // AspNet => Language::new_multi(vec![("<!--", "-->"), ("<%--", "-->")]),
    // ColdFusion => Language::new_multi(vec![("<!---", "--->")]),
    // Coq => Language::new_func(),
    // Handlebars => Language::new_multi(vec![("<!--", "-->"), ("{{!", "}}")])
    //                        .set_quotes(vec![("\"", "\""), ("'", "'")]),
    Idris,

    // C
    ActionScript,
    ColdFusionScript,
    Css,
    Cpp,
    CSharp,
    Dart,
    DeviceTree,
    Go,
    Jai,
    Java,
    JavaScript,
    Jsx,
    Kotlin,
    Less,
    LinkerScript,
    ObjectiveC,
    ObjectiveCpp,
    Qcl,
    Sass,
    Scala,
    Swift,
    TypeScript,
    UnrealScript,
    // END known C
    //
    // Bash style
    CShell,
    Makefile,
    Nim,
    R,
    Toml,
    Yaml,
    Zsh,
    // END bash style
    //
    // HTML style
    Html,
    Polly,
    RubyHtml,

    CoffeeScript,
    D,
    Forth,

    // Pascal,
    // Php => "PHP",
    // Php => SM("#","//""/*", "*/"),
    Python,
    Julia,
    Lisp,
    Lua,
    Sql,
    // END HTML
    //
    // Multi
    // End multi
    //
    //  Single
    Ada,
    Batch,
    Erlang,
    Protobuf,
    Tex,
    VimScript,
    //  End single
    //
    // Standard single + multi
    // Php => Language::new(vec!["#","//"], vec![("/*", "*/")]),
    //
    //
    // Isabelle => Language::new(
    //     vec!["--"],
    //     vec![   ("{*","*}"),
    //             ("(*","*)"),
    //             ("‹","›"),
    //             ("\\<open>", "\\<close>"),
    //         ]
    // ),
    // Json => Language::new_blank(),
    // Markdown => Language::new_blank(),
    // Text => Language::new_blank(),
    //
    // Oz => Language::new_pro(),
    // Prolog => Language::new_pro(),
    //
    // Mustache => Language::new_multi(vec![("{{!", "}}")]),
    // Razor => Language::new_multi(vec![("<!--", "-->"), ("@*", "*@")]),
    //
    // Sml => Language::new_func(),
    // Wolfram => Language::new_func(),
    // OCaml => Language::new_func(),
    Unrecognized,
}
use self::Lang::*;

impl Lang {
    pub fn to_s(&self) -> &str {
        match *self {
            C => "C",
            CCppHeader => "C/C++ Header",
            Rust => "Rust",
            Ruby => "Ruby",
            Haskell => "Haskell",
            Perl => "Perl",
            BourneShell => "Bourne Shell",
            Make => "Make",
            INI => "INI",
            Assembly => "Assembly",
            Yacc => "Yacc",
            Awk => "Awk",
            XML => "XMl",

            CoffeeScript => "CoffeeScript",
            D => "D",
            Forth => "Forth",
            // Pascal => "Pascal",
            //
            // Php => "PHP",
            // Php => SM("#","//""/*", "*/"),
            Python => "Python",
            Julia => "Julia",
            Lisp => "Lisp",
            Lua => "Lua",
            Sql => "SQL",

            Ada => "Ada",
            Batch => "Batch",
            Erlang => "Erlang",
            Protobuf => "Protobuf",
            Tex => "Tex",
            VimScript => "VimL",

            Idris => "Idris",
            ActionScript => "ActionScript",
            ColdFusionScript => "ColdFusionScript",
            Css => "Css",
            Cpp => "Cpp",
            CSharp => "CSharp",
            Dart => "Dart",
            DeviceTree => "DeviceTree",
            Go => "Go",
            Jai => "Jai",
            Java => "Java",
            JavaScript => "JavaScript",
            Jsx => "Jsx",
            Kotlin => "Kotlin",
            Less => "Less",
            LinkerScript => "LinkerScript",
            ObjectiveC => "ObjectiveC",
            ObjectiveCpp => "ObjectiveCpp",
            Qcl => "Qcl",
            Sass => "Sass",
            Scala => "Scala",
            Swift => "Swift",
            TypeScript => "TypeScript",
            UnrealScript => "UnrealScript",
            CShell => "CShell",
            Makefile => "Makefile",
            Nim => "Nim",
            R => "R",
            Toml => "Toml",
            Yaml => "Yaml",
            Zsh => "Zsh",
            Html => "Html",
            Polly => "Polly",
            RubyHtml => "RubyHtml",
            // Php => "PHP",
            // Php => SM("#","//""/*", "*/"),
            Unrecognized => "Unrecognized",
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
        // "c" => C,
        // "h" | "hh" | "hpp" | "hxx" => CCppHeader,
        // "rs" => Rust,
        // "hs" => Haskell,
        // "pl" => Perl,
        // "rb" => Ruby,
        // "makefile" | "mk" => Make,
        // "ini" => INI,
        // "s" | "asm" => Assembly,
        // "y" => Yacc,
        // "awk" => Awk,
        // "xml" => XML,
        //
        // // TODO(cgag): What's the correct extension? Any? Pragma?
        // "sh" => BourneShell,
        "as" => ActionScript,
        "ada" | "adb" | "ads" | "pad" => Ada,
        // "asa" | "asp" => Asp,
        // "asax" | "ascx" | "asmx" | "aspx" | "master" | "sitemap" | "webinfo" => AspNet,
        // "bash" | "sh" => Bash,
        "bat" | "btm" | "cmd" => Batch,
        "c" | "ec" | "pgc" => C,
        "cc" | "cpp" | "cxx" | "c++" | "pcc" => Cpp,
        "cfc" => ColdFusionScript,
        // "cfm" => ColdFusion,
        // "clj" => Clojure,
        "coffee" => CoffeeScript,
        "cs" => CSharp,
        // "cshtml" => Razor,
        "csh" => CShell,
        "css" => Css,
        "d" => D,
        "dart" => Dart,
        "dts" | "dtsi" => DeviceTree,
        "el" | "lisp" | "lsp" => Lisp,
        "erl" | "hrl" => Erlang,
        "4th" | "forth" | "fr" | "frt" | "fth" | "f83" | "fb" | "fpm" | "e4" | "rx" | "ft" => Forth,
        // "f" | "for" | "ftn" | "f77" | "pfo" => FortranLegacy,
        // "f03" | "f08" | "f90" | "f95" => FortranModern,
        "go" => Go,
        // "hbs" | "handlebars" => Handlebars,
        "h" | "hh" | "hpp" | "hxx" => CCppHeader,
        "hs" => Haskell,
        "html" => Html,
        "idr" | "lidr" => Idris,
        // "in" => Autoconf,
        "jai" => Jai,
        "java" => Java,
        "jl" => Julia,
        "js" => JavaScript,
        // "json" => Json,
        "jsx" => Jsx,
        "kt" | "kts" => Kotlin,
        "lds" => LinkerScript,
        "less" => Less,
        "lua" => Lua,
        "m" => ObjectiveC,
        // "markdown" | "md" => Markdown,
        // "ml" | "mli" => OCaml,
        "mm" => ObjectiveCpp,
        "makefile" => Makefile,
        // "mustache" => Mustache,
        "nim" => Nim,
        // "nb" | "wl" => Wolfram,
        // "oz" => Oz,
        // "p" | "pro" => Prolog,
        // "pas" => Pascal,
        // "php" => Php,
        "pl" => Perl,
        "qcl" => Qcl,
        // "text" | "txt" => Text,
        "polly" => Polly,
        "proto" => Protobuf,
        "py" => Python,
        "r" => R,
        "rake" | "rb" => Ruby,
        "rhtml" => RubyHtml,
        "rs" => Rust,
        "s" | "asm" => Assembly,
        "sass" | "scss" => Sass,
        "sc" | "scala" => Scala,
        // "sml" => Sml,
        "sql" => Sql,
        "swift" => Swift,
        "tex" | "sty" => Tex,
        "toml" => Toml,
        "ts" => TypeScript,
        // "thy" => Isabelle,
        "uc" | "uci" | "upkg" => UnrealScript,
        // "v" => Coq,
        "vim" => VimScript,
        "xml" => XML,
        "yaml" | "yml" => Yaml,
        "zsh" => Zsh,

        // Probably dumb to just default to C.
        _ => Unrecognized,
    }
}

enum ConfigTuple<'a> {
    // Single only
    SO(&'a str),
    // MultiOnly
    MO(&'a str, &'a str),
    // Single + Multi
    SM(&'a str, &'a str, &'a str),
}
use self::ConfigTuple::*;
pub fn counter_config_for_lang<'a>(lang: &Lang) -> LineConfig<'a> {

    let c_style = SM("//", "/*", "*/");
    let sh_style = SO("#");
    let html_style = MO("<!--", "-->");

    let ctuple = match *lang {
        Haskell => SM("--", "{-", "-}"),
        Idris => SM("--", "{-", "-}"),
        // which one is right? = or =pod?
        Lang::Perl => SM("#", "=pod", "=cut"),
        // Perl => SM("#""=", "=cut"),
        Lang::INI => SO(";"),

        CoffeeScript => SM("#", "###", "###"),
        D => SM("//", "/*", "*/"),
        Forth => SM("\\", "(", ")"),
        Python => SM("#", "'\''", "'\''"),
        Julia => SM("#", "#=", "=#"),
        Lisp => SM(";", "#|", "|#"),
        Lua => SM("--", "--[[", "]]"),
        Ruby => SM("#", "=begin", "=end"),
        Sql => SM("--", "/*", "*/"),

        Ada => SO("--"),
        Batch => SO("REM"),
        Erlang => SO("%"),
        Protobuf => SO("//"),
        Tex => SO("%"),
        VimScript => SO("\""),

        // Pascal?
        // TODO(cgag): Well, some architectures use ;, @, |, etc.
        // Need a way to specify more than one possible comment char.
        Assembly => SM("#", "/*", "*/"),
        // TODO(cgag): Welp, single is not always necessary
        Html | Polly | RubyHtml | XML => html_style,
        BourneShell | Lang::Make | Lang::Awk | CShell | Makefile | Nim | R | Toml | Yaml | Zsh => {
            sh_style
        }
        // TODO(cgag): not 100% that yacc belongs here.
        C | CCppHeader | Rust | Lang::Yacc | ActionScript | ColdFusionScript | Css | Cpp |
        CSharp | Dart | DeviceTree | Go | Jai | Java | JavaScript | Jsx | Kotlin | Less |
        LinkerScript | ObjectiveC | ObjectiveCpp | Qcl | Sass | Scala | Swift | TypeScript |
        UnrealScript | Unrecognized => c_style,
    };

    match ctuple {
        SM(single, start, end) => {
            LineConfig::SingleMulti {
                single_start: single,
                multi_start: start,
                multi_end: end,
            }
        }
        SO(single) => LineConfig::SingleOnly { single_start: single },
        MO(start, end) => {
            LineConfig::MultiOnly {
                multi_start: start,
                multi_end: end,
            }
        }
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
        LineConfig::SingleOnly { single_start: single } => count_single_only(filepath, single),
        LineConfig::SingleMulti { single_start: single, multi_start, multi_end } => {
            count_single_multi(filepath, single, multi_start, multi_end)
        }
        LineConfig::MultiOnly { multi_start: mstart, multi_end: mend } => {
            count_multi_only(filepath, mstart, mend)
        }
    }
}

pub fn count_single_only(filepath: &str, single_start: &str) -> Count {
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

pub fn count_multi_only(filepath: &str, multi_start: &str, multi_end: &str) -> Count {
    // this is a duplicate of count_single_multi without the check for single comment.
    // Basically removes one branch.  Probably pointless: benchmark.
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


pub fn count_single_multi(filepath: &str,
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
