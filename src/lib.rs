pub mod lines;

extern crate regex;
extern crate memmap;
extern crate memchr;

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
    },
    Everything {
        singles: Vec<&'a str>,
        multies: Vec<(&'a str, &'a str)>,
    },
}

// Do any languages actually use utf8 chars as comment chars?
// We can probably do something with the encoding crate where we decode
// as ascii, and then use unsafe_from_utf8. If decoding fails,
// we catch it and just use the safe from_utf8 as we're doing now.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Lang {
    ActionScript,
    Ada,
    Agda,
    Asp,
    AspNet,
    Assembly,
    Autoconf,
    Awk,
    Batch,
    BourneShell,
    C,
    CCppHeader,
    CSharp,
    CShell,
    Clojure,
    CoffeeScript,
    ColdFusion,
    ColdFusionScript,
    Coq,
    Cpp,
    Css,
    CUDA,
    CUDAHeader,
    D,
    Dart,
    DeviceTree,
    Erlang,
    Forth,
    FortranLegacy,
    FortranModern,
    FSharp,
    Glsl,
    Go,
    Handlebars,
    Haskell,
    Hex,
    Html,
    INI,
    Idris,
    IntelHex,
    Isabelle,
    Jai,
    Java,
    JavaScript,
    Json,
    Jsx,
    Julia,
    Kotlin,
    Less,
    LinkerScript,
    Lean,
    Lisp,
    Lua,
    Make,
    Makefile,
    Markdown,
    Mustache,
    Nim,
    OCaml,
    ObjectiveC,
    ObjectiveCpp,
    Oz,
    Pascal,
    Perl,
    Php,
    Polly,
    Prolog,
    Protobuf,
    Python,
    Qcl,
    R,
    Razor,
    ReStructuredText,
    Ruby,
    RubyHtml,
    Rust,
    SaltStack,
    Sass,
    Scala,
    Sml,
    Sql,
    Swift,
    Tcl,
    Tex,
    Text,
    Toml,
    TypeScript,
    UnrealScript,
    VimScript,
    Wolfram,
    XML,
    Yacc,
    Yaml,
    Zsh,

    Unrecognized,
}
use self::Lang::*;

impl Lang {
    pub fn to_s(&self) -> &str {
        match *self {
            ActionScript => "ActionScript",
            Ada => "Ada",
            Agda => "Agda",
            Asp => "ASP",
            AspNet => "ASP.NET",
            Assembly => "Assembly",
            Autoconf => "Autoconf",
            Awk => "Awk",
            Batch => "Batch",
            BourneShell => "Bourne Shell",
            C => "C",
            CCppHeader => "C/C++ Header",
            CSharp => "C#",
            CShell => "C Shell",
            Clojure => "Clojure",
            CoffeeScript => "CoffeeScript",
            ColdFusion => "ColdFusion",
            ColdFusionScript => "ColdFusionScript",
            Coq => "Coq",
            Cpp => "C++",
            Css => "CSS",
            CUDA => "CUDA",
            CUDAHeader => "CUDA Header",
            D => "D",
            Dart => "Dart",
            DeviceTree => "DeviceTree",
            Erlang => "Erlang",
            Forth => "Forth",
            FortranLegacy => "FORTRAN Legacy",
            FortranModern => "FORTRAN Modern",
            FSharp => "F#",
            Glsl => "GLSL",
            Go => "Go",
            Handlebars => "Handlebars",
            Haskell => "Haskell",
            Hex => "Hex",
            Html => "HTML",
            INI => "INI",
            Idris => "Idris",
            IntelHex => "Intel Hex",
            Isabelle => "Isabelle",
            Jai => "Jai",
            Java => "Java",
            JavaScript => "JavaScript",
            Json => "JSON",
            Jsx => "Jsx",
            Julia => "Julia",
            Kotlin => "Kotlin",
            Less => "Less",
            LinkerScript => "LinkerScript",
            Lean => "Lean",
            Lisp => "Lisp",
            Lua => "Lua",
            Make => "Make",
            Makefile => "Makefile",
            Markdown => "Markdown",
            Mustache => "Mustache",
            Nim => "Nim",
            OCaml => "OCaml",
            ObjectiveC => "Objective-C",
            ObjectiveCpp => "Objective-C++",
            Oz => "Oz",
            Pascal => "Pascal",
            Perl => "Perl",
            Php => "PHP",
            Polly => "Polly",
            Prolog => "Prolog",
            Protobuf => "Protobuf",
            Python => "Python",
            Qcl => "Qcl",
            R => "R",
            Razor => "Razor",
            ReStructuredText => "reStructuredText",
            Ruby => "Ruby",
            RubyHtml => "RubyHtml",
            Rust => "Rust",
            SaltStack => "SaltStack",
            Sass => "Sass",
            Scala => "Scala",
            Sml => "SML",
            Sql => "SQL",
            Swift => "Swift",
            Tcl => "Tcl",
            Tex => "TeX",
            Text => "Plain Text",
            Toml => "Toml",
            TypeScript => "TypeScript",
            UnrealScript => "UnrealScript",
            VimScript => "VimL",
            Wolfram => "Wolfram",
            XML => "XML",
            Yacc => "Yacc",
            Yaml => "YAML",
            Zsh => "Z Shell",

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
    let file_name_lower = path.file_name()
        .expect("no filename?")
        .to_str()
        .expect("to_str")
        .to_lowercase();

    let ext = if file_name_lower.contains("makefile") {
        String::from("makefile")
    } else {
        match path.extension() {
            Some(os_str) => os_str.to_str().unwrap().to_lowercase(),
            None => file_name_lower,
        }
    };

    // NOTE(cgag): while we lifted most of this from tokei, we support a few
    // more extensions in some places, can't just assume it's the same.
    match &*ext {
        "4th" | "forth" | "fr" | "frt" | "fth" | "f83" | "fb" | "fpm" | "e4" | "rx" | "ft" => Forth,
        "ada" | "adb" | "ads" | "pad" => Ada,
        "agda" => Agda,
        "as" => ActionScript,
        "awk" => Awk,
        "bat" | "btm" | "cmd" => Batch,
        "c" | "ec" | "pgc" => C,
        "cc" | "cpp" | "cxx" | "c++" | "pcc" => Cpp,
        "cfc" => ColdFusionScript,
        "coffee" => CoffeeScript,
        "cs" => CSharp,
        "csh" => CShell,
        "css" => Css,
        "cu" => CUDA,
        "cuh" => CUDAHeader,
        "d" => D,
        "dart" => Dart,
        "dts" | "dtsi" => DeviceTree,
        "el" | "lisp" | "lsp" => Lisp,
        "erl" | "hrl" => Erlang,
        "fs" | "fsx" => FSharp,
        "vert" | "tesc" | "tese" | "geom" | "frag" | "comp" => Glsl,
        "go" => Go,
        "h" | "hh" | "hpp" | "hxx" => CCppHeader,
        "hbs" | "handlebars" => Handlebars,
        "hs" => Haskell,
        "html" => Html,
        "idr" | "lidr" => Idris,
        "ini" => INI,
        "jai" => Jai,
        "java" => Java,
        "jl" => Julia,
        "js" => JavaScript,
        "jsx" => Jsx,
        "kt" | "kts" => Kotlin,
        "lds" => LinkerScript,
        "lean" | "hlean" => Lean,
        "less" => Less,
        "lua" => Lua,
        "m" => ObjectiveC,
        "ml" | "mli" => OCaml,
        "nb" | "wl" => Wolfram,
        "sh" => BourneShell,
        "asa" | "asp" => Asp,
        "asax" | "ascx" | "asmx" | "aspx" | "master" | "sitemap" | "webinfo" => AspNet,
        "in" => Autoconf,
        "clj" | "cljs" | "cljc" => Clojure,

        "f" | "for" | "ftn" | "f77" | "pfo" => FortranLegacy,
        "f03" | "f08" | "f90" | "f95" => FortranModern,
        "makefile" | "mk" => Makefile,
        "mm" => ObjectiveCpp,
        "nim" => Nim,
        "php" => Php,
        "pl" => Perl,
        "qcl" => Qcl,
        "cshtml" => Razor,
        "mustache" => Mustache,
        "oz" => Oz,
        "p" | "pro" => Prolog,
        "pas" => Pascal,
        "hex" => Hex,
        "ihex" => IntelHex,
        "json" => Json,
        "markdown" | "md" => Markdown,
        "rst" => ReStructuredText,
        "text" | "txt" => Text,

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
        "sls" => SaltStack,
        "sml" => Sml,
        "sql" => Sql,
        "swift" => Swift,
        "tcl" => Tcl,
        "tex" | "sty" => Tex,
        "toml" => Toml,
        "ts" => TypeScript,
        "thy" => Isabelle,
        "uc" | "uci" | "upkg" => UnrealScript,
        "v" => Coq,
        "vim" => VimScript,
        "xml" => XML,
        "yaml" | "yml" => Yaml,
        "y" => Yacc,
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
    // Everything (multiple singles, multiple multiline)
    EV(Vec<&'a str>, Vec<(&'a str, &'a str)>),
}
use self::ConfigTuple::*;

const UNLIKELY: &'static str = "SLkJJJJJ<!*$(!*&)(*@^#$K8K!(*76(*&(38j8";

pub fn counter_config_for_lang<'a>(lang: &Lang) -> LineConfig<'a> {

    let c_style = SM("//", "/*", "*/");
    let html_style = MO("<!--", "-->");
    let ml_style = MO("(*", "*)");
    // TODO(cgag): Find a less dumb way to do this.
    let no_comments = SO(UNLIKELY);
    let sh_style = SO("#");
    let prolog_style = SM("%", "/*", "*/");

    let ctuple = match *lang {
        Ada => SO("--"),
        Batch => SO("REM"),
        Erlang | Tex => SO("%"),
        FortranModern => SO("!"),
        Haskell | Idris | Agda => SM("--", "{-", "-}"),
        INI => SO(";"),
        Protobuf => SO("//"),
        VimScript => SO("\""),

        // TODO(cgag): Well, some architectures use ;, @, |, etc.  Figure out something
        // better?
        Assembly => SM("#", "/*", "*/"),
        CoffeeScript => SM("#", "###", "###"),
        D => SM("//", "/*", "*/"),
        Forth => SM("\\", "(", ")"),
        FSharp => SM("//", "(*", "*)"),
        Julia => SM("#", "#=", "=#"),
        Lisp => SM(";", "#|", "|#"),
        Lean => SM("--", "/-", "-/"),
        Lua => SM("--", "--[[", "]]"),
        // which one is right? = or =pod?
        // Perl => SM("#""=", "=cut"),
        Perl => SM("#", "=pod", "=cut"),
        Python => SM("#", "'''", "'''"),
        Ruby => SM("#", "=begin", "=end"),
        Sql => SM("--", "/*", "*/"),

        ColdFusion => MO("<!---", "--->"),
        Mustache => MO("{{!", "}}"),

        Asp => EV(vec!["'", "REM"], vec![]),
        AspNet => EV(vec![UNLIKELY], vec![("<!--", "-->"), ("<%--", "-->")]),
        Autoconf => EV(vec!["#", "dnl"], vec![]),
        Clojure => EV(vec![";", "#"], vec![]),
        FortranLegacy => EV(vec!["c", "C", "!", "*"], vec![]),
        Handlebars => EV(vec![UNLIKELY], vec![("<!--", "-->"), ("{{!", "}}")]),
        Php => EV(vec!["#", "//"], vec![("/*", "*/")]),
        Isabelle => {
            EV(vec!["--"],
               vec![("{*", "*}"), ("(*", "*)"), ("‹", "›"), ("\\<open>", "\\<close>")])
        }
        Razor => EV(vec![UNLIKELY], vec![("<!--", "-->"), ("@*", "*@")]),
        Pascal => EV(vec!["//", "(*"], vec![("{", "}")]),
        Text | Markdown | Json | IntelHex | Hex | ReStructuredText => no_comments,

        Oz | Prolog => prolog_style,

        Coq | Sml | Wolfram | OCaml => ml_style,

        Html | Polly | RubyHtml | XML => html_style,

        BourneShell | Make | Awk | CShell | Makefile | Nim | R | SaltStack | Tcl | Toml |
        Yaml | Zsh => sh_style,

        // TODO(cgag): not 100% sure that yacc belongs here.
        C | CCppHeader | Rust | Yacc | ActionScript | ColdFusionScript | Css | Cpp | CUDA |
        CUDAHeader | CSharp | Dart | DeviceTree | Glsl | Go | Jai | Java | JavaScript | Jsx |
        Kotlin | Less | LinkerScript | ObjectiveC | ObjectiveCpp | Qcl | Sass | Scala | Swift |
        TypeScript | UnrealScript => c_style,

        Unrecognized => unreachable!(),
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
        EV(singles, multies) => {
            LineConfig::Everything {
                singles: singles,
                multies: multies,
            }
        }
    }
}

struct ByteLinesState<'a> {
    buf: &'a [u8],
    pos: usize,
}

struct ByteLines<'a>(&'a [u8]);

impl<'a> ByteLines<'a> {
    fn lines(&self) -> ByteLinesState {
        ByteLinesState {
            buf: self.0,
            pos: 0,
        }
    }
}

impl<'a> Iterator for ByteLinesState<'a> {
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
        LineConfig::SingleOnly { single_start } => count_single(filepath, single_start),
        LineConfig::SingleMulti { single_start, multi_start, multi_end } => {
            count_single_multi(filepath, single_start, multi_start, multi_end)
        }
        LineConfig::MultiOnly { multi_start, multi_end } => {
            count_multi(filepath, multi_start, multi_end)
        }
        LineConfig::Everything { singles, multies } => count_everything(filepath, singles, multies),
    }
}

pub fn count_single(filepath: &str, single_start: &str) -> Count {
    let fmmap = match Mmap::open_path(filepath, Protection::Read) {
        Ok(mmap) => mmap,
        Err(_) => {
            return Count::default();
        }
    };
    let bytes: &[u8] = unsafe { fmmap.as_slice() };

    let mut c = Count::default();

    for byte_line in ByteLines(bytes).lines() {
        let line = match std::str::from_utf8(byte_line) {
            Ok(s) => s,
            Err(_) => return Count::default(),
        };
        c.lines += 1;

        let trimmed = line.trim_left();
        if trimmed.is_empty() {
            c.blank += 1;
        } else if trimmed.starts_with(single_start) {
            c.comment += 1;
        } else {
            c.code += 1;
        }
    }

    c
}

pub fn count_multi(filepath: &str, multi_start: &str, multi_end: &str) -> Count {
    // this is a duplicate of count_single_multi without the check for single comment.
    // Basically removes one branch.  Probably pointless: benchmark.
    let multiline_start = multi_start;
    let multiline_end = multi_end;

    let fmmap = match Mmap::open_path(filepath, Protection::Read) {
        Ok(mmap) => mmap,
        Err(_) => {
            return Count::default();
        }
    };
    let bytes: &[u8] = unsafe { fmmap.as_slice() };

    let mut c = Count::default();
    let mut in_comment = false;

    for byte_line in ByteLines(bytes).lines() {
        let line = match std::str::from_utf8(byte_line) {
            Ok(s) => s,
            Err(_) => return Count::default(),
        };
        c.lines += 1;

        let trimmed = line.trim_left();
        if trimmed.is_empty() {
            c.blank += 1;
            continue;
        };

        if !trimmed.contains(multiline_start) && !trimmed.contains(multiline_end) {
            if in_comment {
                c.comment += 1;
            } else {
                c.code += 1;
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

            if !in_comment && pos + start_len <= trimmed.len() &&
               &trimmed[pos..(pos + start_len)] == multi_start {
                pos += start_len;
                in_comment = true;
            } else if in_comment && pos + end_len <= trimmed.len() &&
               &trimmed[pos..(pos + end_len)] == multi_end {
                pos += end_len;
                in_comment = false;
                // TODO(cgag): should we bother handling whitespace here?
            } else if !in_comment {
                found_code = true;
                pos += 1;
            } else {
                pos += 1;
            }
        }

        if found_code {
            c.code += 1;
        } else {
            c.comment += 1;
        }
    }

    c
}

// TODO(cgag): prune down to just count everything, count_single, count_multi?
pub fn count_everything<'a>(filepath: &str,
                            singles: Vec<&'a str>,
                            multies: Vec<(&'a str, &'a str)>)
                            -> Count {

    let mut single_iter = singles.iter();
    // TODO(cgag): actually i think if we just had multiple multiline comments
    // and no single line comments that this could indeed fail.  Need to potentially
    // get first one from the multies.
    let first = single_iter.next().expect("There should always be at least one?");
    let mut total_count = count_single(filepath, first);

    for single in single_iter {
        let count = count_single(filepath, single);
        total_count.comment += count.comment;
        // subtract out comments that were counted as code in previous counts
        total_count.code -= count.comment;
    }

    for (multi_start, multi_end) in multies {
        let count = count_multi(filepath, multi_start, multi_end);
        total_count.comment += count.comment;
        // subtract out comments that were counted as code in previous counts
        total_count.code -= count.comment;
    }

    total_count
}

pub fn count_single_multi(filepath: &str,
                          single_start: &str,
                          multi_start: &str,
                          multi_end: &str)
                          -> Count {

    let fmmap = match Mmap::open_path(filepath, Protection::Read) {
        Ok(mmap) => mmap,
        Err(_) => {
            return Count::default();
        }
    };
    let bytes: &[u8] = unsafe { fmmap.as_slice() };

    let mut c = Count::default();
    let mut in_comment = false;

    for byte_line in ByteLines(bytes).lines() {
        let line = match std::str::from_utf8(byte_line) {
            Ok(s) => s,
            Err(_) => return Count::default(),
        };
        c.lines += 1;

        let trimmed = line.trim_left();
        if trimmed.is_empty() {
            c.blank += 1;
            continue;
        };

        if !in_comment && trimmed.starts_with(single_start) {
            c.comment += 1;
            continue;
        }

        if !(trimmed.contains(multi_start) || trimmed.contains(multi_end)) {
            if in_comment {
                c.comment += 1;
            } else {
                c.code += 1;
            }
            continue;
        }

        let start_len = multi_start.len();
        let end_len = multi_end.len();
        let trimmed_len = trimmed.len();

        let mut pos = 0;
        let mut found_code = false;
        'outer: while pos < trimmed_len {
            // TODO(cgag): must be a less stupid way to do this.  At the
            // very least don't recalculate max over and over.  LLVM probably
            // optimizes this but it seems dumb to depend on it?
            for i in pos..(pos + cmp::max(start_len, end_len) + 1) {
                if !trimmed.is_char_boundary(i) {
                    pos += 1;
                    continue 'outer;
                }
            }

            if !in_comment && pos + start_len <= trimmed_len &&
               &trimmed[pos..(pos + start_len)] == multi_start {
                pos += start_len;
                in_comment = true;
            } else if in_comment && pos + end_len <= trimmed_len &&
               &trimmed[pos..(pos + end_len)] == multi_end {
                pos += end_len;
                in_comment = false;
                // TODO(cgag): should we bother handling whitespace here?
            } else if !in_comment {
                found_code = true;
                pos += 1;
            } else {
                pos += 1;
            }
        }

        if found_code {
            c.code += 1;
        } else {
            c.comment += 1;
        }
    }

    c
}
