extern crate loc;

#[macro_use]
extern crate clap;
extern crate deque;
extern crate num_cpus;
extern crate regex;
extern crate ignore;
extern crate edit_distance;

use clap::{Arg, App, AppSettings};
use ignore::WalkBuilder;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::thread;
use std::str::FromStr;

use deque::{Stealer, Stolen};
use regex::Regex;
use edit_distance::edit_distance as distance;

use loc::*;

enum Work {
    File(String),
    Quit,
}

struct Worker {
    chan: Stealer<Work>,
}

#[derive(Clone)]
struct FileCount {
    path: String,
    lang: Lang,
    count: Count,
}

// This concurrency pattern ripped directly from ripgrep
impl Worker {
    fn run(self) -> Vec<FileCount> {
        let mut v: Vec<FileCount> = vec![];
        loop {
            match self.chan.steal() {
                // What causes these?
                Stolen::Empty | Stolen::Abort => continue,
                Stolen::Data(Work::Quit) => break,
                Stolen::Data(Work::File(path)) => {
                    let lang = lang_from_ext(&path);
                    if lang != Lang::Unrecognized {
                        let count = count(&path);
                        v.push(FileCount {
                            lang: lang,
                            path: path,
                            count: count,
                        });
                    }
                }
            };
        }
        v
    }
}

#[derive(PartialEq)]
enum Sort {
    Code,
    Comment,
    Blank,
    Lines,
    // The following options are only used if the --files flag is NOT present
    Language,
    Files,
}

impl FromStr for Sort {
    /// A Some variant indicates a suggested value -- the given value was close (perhaps
    /// because of a typo) to a valid value. None indicates that the given value was not
    /// close to a correct value.
    type Err = Option<String>;
    fn from_str(s: &str) -> Result<Sort, Self::Err> {
        match s {
            "blank"    | "Blank" => Ok(Sort::Blank),
            "code"     | "Code" => Ok(Sort::Code),
            "comment"  | "Comment" => Ok(Sort::Comment),
            "lines"    | "Lines" => Ok(Sort::Lines),
            "language" | "Language" => Ok(Sort::Language),
            "files"    | "Files" => Ok(Sort::Files),
            s if distance(&s.to_lowercase(), "blank") <= 2 => Err(Some("Blank".into())),
            s if distance(&s.to_lowercase(), "code") <= 2 => Err(Some("Code".into())),
            s if distance(&s.to_lowercase(), "comment") <= 2 => Err(Some("Comment".into())),
            s if distance(&s.to_lowercase(), "lines") <= 2 => Err(Some("Lines".into())),
            s if distance(&s.to_lowercase(), "language") <= 2 => Err(Some("Language".into())),
            s if distance(&s.to_lowercase(), "files") <= 2 => Err(Some("Files".into())),
            _ => Err(None)
        }
    }
}

// TODO(cgag): tune smallvec array sizes
fn main() {
    let matches = App::new("loc")
        .global_settings(&[AppSettings::ColoredHelp])
        .version(crate_version!())
        .author("Curtis Gagliardi <curtis@curtis.io>")
        .about("counts things quickly hopefully")
        .arg(Arg::with_name("exclude")
            .required(false)
            .multiple(true)
            .long("exclude")
            .value_name("REGEX")
            .takes_value(true)
            .help("Rust regex of files to exclude"))
        .arg(Arg::with_name("include")
            .required(false)
            .multiple(true)
            .long("include")
            .value_name("REGEX")
            .takes_value(true)
            .help("Rust regex matching files to include. Anything not matched will be excluded"))
        .arg(Arg::with_name("files")
             .required(false)
             .long("files")
             .takes_value(false)
             .help("Show stats for individual files"))
        .arg(Arg::with_name("sort")
            .required(false)
            .long("sort")
            .value_name("COLUMN")
            .takes_value(true)
            .help("Column to sort by"))
        .arg(Arg::with_name("unrestricted")
             .required(false)
             .multiple(true)
             .long("unrestricted")
             .short("u")
             .takes_value(false)
             .help("A single -u won't respect .gitignore (etc.) files. Two -u flags will additionally count hidden files and directories."))
        .arg(Arg::with_name("target")
            .multiple(true)
            .help("File or directory to count (multiple arguments accepted)"))
        .get_matches();

    let targets = match matches.values_of("target") {
        Some(targets) => targets.collect(),
        None => vec!["."]
    };

    let sort: Sort = match matches.value_of("sort") {
        Some(string) => match Sort::from_str(string) {
            Ok(sort) => sort,
            Err(err) => {
                if let Some(suggestion) = err {
                    println!("Error: invalid value for --sort: '{}', perhaps you meant '{}'?",
                             string, suggestion);
                } else {
                    println!("Error: invalid value for --sort: '{}'", string);
                }
                println!(" Hint: legal values are Code, Comment, Blank, Lines, Language, \
                          and Files");
                return
            },
        },
        // Default to sorting by lines of code
        None => Sort::Code,
    };

    let by_file: bool = matches.is_present("files");

    if by_file && (sort == Sort::Language || sort == Sort::Files) {
        println!("Error: cannot sort by Language or Files when --files is present");
        return
    }

    let (use_ignore, ignore_hidden) = match matches.occurrences_of("unrestricted") {
        0 => (true,  true),
        1 => (false, true),
        2 => (false, false),
        _ => (false, false),
    };
    let exclude_regex = match matches.values_of("exclude") {
        Some(regex_strs) => {
            let combined_regex = regex_strs.map(|r| format!("({})", r)).collect::<Vec<String>>().join("|");
            match Regex::new(&combined_regex) {
                Ok(r) => Some(r),
                Err(e) => {
                    println!("Error processing exclude regex: {}", e);
                    std::process::exit(1);
                }
            }
        }
        None => None,
    };
    let include_regex = match matches.values_of("include") {
        Some(regex_strs) => {
            let combined_regex = regex_strs.map(|r| format!("({})", r)).collect::<Vec<String>>().join("|");
            match Regex::new(&combined_regex) {
                Ok(r) => Some(r),
                Err(e) => {
                    println!("Error processing include regex: {}", e);
                    std::process::exit(1);
                }
            }
        }
        None => None,
    };

    let threads = num_cpus::get();
    let mut workers = vec![];
    let (workq, stealer) = deque::new();
    for _ in 0..threads {
        let worker = Worker { chan: stealer.clone() };
        workers.push(thread::spawn(|| worker.run()));
    }

    for target in targets {
        // TODO(cgag): use WalkParallel?
        let walker = WalkBuilder::new(target).ignore(use_ignore)
                                             .git_ignore(use_ignore)
                                             .git_exclude(use_ignore)
                                             .hidden(ignore_hidden)
                                             .build();
        let files = walker
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().expect("no filetype").is_file())
            .map(|entry| String::from(entry.path().to_str().unwrap()))
            .filter(|path| match include_regex {
                None => true,
                Some(ref include) => include.is_match(path),
            })
            .filter(|path| match exclude_regex {
                None => true,
                Some(ref exclude) => !exclude.is_match(path),
            });

        for path in files {
            workq.push(Work::File(path));
        }
    }

    for _ in 0..workers.len() {
        workq.push(Work::Quit);
    }

    let mut filecounts: Vec<FileCount> = Vec::new();
    for worker in workers {
        filecounts.extend(worker.join().unwrap().iter().cloned())
    }

    // TODO(cgag): use insecure hashmaps or something
    let mut by_lang: HashMap<Lang, Vec<FileCount>> = HashMap::new();
    for fc in filecounts {
        match by_lang.entry(fc.lang) {
            Entry::Occupied(mut elem) => elem.get_mut().push(fc),
            Entry::Vacant(elem) => {
                elem.insert(vec![fc]);
            }
        };
    }

    let linesep = str_repeat("-", 80);

    if by_file {
        // print breakdown for each individual file
        println!("{}", linesep);
        println!(" {0: <17} {1: >8} {2: >12} {3: >12} {4: >12} {5: >12}",
                 "Language",
                 "Files",
                 "Lines",
                 "Blank",
                 "Comment",
                 "Code");
        println!("{}", linesep);

        // TODO(cgag): do the summing first, so we can do additional sorting
        // by totals.
        for (lang, mut filecounts) in by_lang {
            let mut total = Count::default();
            for fc in &filecounts {
                total.merge(&fc.count);
            }

            println!("{}", linesep);
            println!(" {0: <17} {1: >8} {2: >12} {3: >12} {4: >12} {5: >12}",
                     lang,
                     filecounts.len(),
                     total.lines,
                     total.blank,
                     total.comment,
                     total.code);

            match sort {
                Sort::Code    => filecounts.sort_by(|fc1, fc2| fc2.count.code.cmp(&fc1.count.code)),
                Sort::Comment => filecounts.sort_by(|fc1, fc2| fc2.count.comment.cmp(&fc1.count.comment)),
                Sort::Blank   => filecounts.sort_by(|fc1, fc2| fc2.count.blank.cmp(&fc1.count.blank)),
                Sort::Lines   => filecounts.sort_by(|fc1, fc2| fc2.count.lines.cmp(&fc1.count.lines)),
                // No sorting by language or files here when using --files. This should
                // have been checked above.
                Sort::Language | Sort::Files => panic!("Sorting by language or files \
                                                        when using the --sort flag"),
            }

            println!("{}", linesep);
            for fc in filecounts {
                println!("|{0: <25} {1: >12} {2: >12} {3: >12} {4: >12}",
                         last_n_chars(&fc.path, 25),
                         fc.count.lines,
                         fc.count.blank,
                         fc.count.comment,
                         fc.count.code);
            }
        }
    } else {
        // print summary by language
        let mut lang_totals: HashMap<&Lang, LangTotal> = HashMap::new();
        for (lang, filecounts) in &by_lang {
            let mut lang_total = Count::default();
            for fc in filecounts {
                lang_total.merge(&fc.count);
            }
            lang_totals.insert(lang,
                               LangTotal {
                                   files: filecounts.len() as u32,
                                   count: lang_total,
                               });
        }

        let mut totals_by_lang = lang_totals.iter().collect::<Vec<(&&Lang, &LangTotal)>>();
        match sort {
            Sort::Language => totals_by_lang.sort_by(|&(l1, _), &(l2, _)| l1.to_s().cmp(l2.to_s())),
            Sort::Files    => totals_by_lang.sort_by(|&(_, c1), &(_, c2)| c2.files.cmp(&c1.files)),
            Sort::Code     => totals_by_lang.sort_by(|&(_, c1), &(_, c2)| c2.count.code.cmp(&c1.count.code)),
            Sort::Comment  => totals_by_lang.sort_by(|&(_, c1), &(_, c2)| c2.count.comment.cmp(&c1.count.comment)),
            Sort::Blank    => totals_by_lang.sort_by(|&(_, c1), &(_, c2)| c2.count.blank.cmp(&c1.count.blank)),
            Sort::Lines    => totals_by_lang.sort_by(|&(_, c1), &(_, c2)| c2.count.lines.cmp(&c1.count.lines)),
        }

        print_totals_by_lang(&linesep, &totals_by_lang);
    }

}

// TODO(cgag): i think this is in the stdlib
fn last_n_chars(s: &str, n: usize) -> String {
    if s.len() <= n {
        return String::from(s);
    }
    s.chars().skip(s.len() - n).collect::<String>()
}

// TODO(cgag): i think this is in the stdlib
fn str_repeat(s: &str, n: usize) -> String {
    std::iter::repeat(s).take(n).collect::<Vec<_>>().join("")
}

fn print_totals_by_lang(linesep: &str, totals_by_lang: &[(&&Lang, &LangTotal)]) {
    println!("{}", linesep);
    println!(" {0: <17} {1: >8} {2: >12} {3: >12} {4: >12} {5: >12}",
             "Language",
             "Files",
             "Lines",
             "Blank",
             "Comment",
             "Code");
    println!("{}", linesep);

    for &(lang, total) in totals_by_lang {
        println!(" {0: <17} {1: >8} {2: >12} {3: >12} {4: >12} {5: >12}",
                 lang,
                 total.files,
                 total.count.lines,
                 total.count.blank,
                 total.count.comment,
                 total.count.code);
    }

    let mut totals = LangTotal {
        files: 0,
        count: Count::default(),
    };
    for &(_, total) in totals_by_lang {
        totals.files         += total.files;
        totals.count.code    += total.count.code;
        totals.count.blank   += total.count.blank;
        totals.count.comment += total.count.comment;
        totals.count.lines   += total.count.lines;
    }

    println!("{}", linesep);
    println!(" {0: <17} {1: >8} {2: >12} {3: >12} {4: >12} {5: >12}",
             "Total",
             totals.files,
             totals.count.lines,
             totals.count.blank,
             totals.count.comment,
             totals.count.code);
    println!("{}", linesep);
}
