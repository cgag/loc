extern crate loc;

extern crate clap;
extern crate deque;
extern crate itertools;
extern crate num_cpus;
extern crate walkdir;
extern crate regex;

use clap::{Arg, App, AppSettings};
use walkdir::WalkDir;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::thread;

use deque::{Stealer, Stolen};
use regex::Regex;

use loc as c;

enum Work {
    File(String),
    Quit,
}

struct Worker {
    chan: Stealer<Work>,
}

// TODO(cgag): name this (lang, path, count) tuple concept and make it a struct
impl Worker {
    fn run(self) -> Vec<(c::Lang, String, c::Count)> {
        let mut v: Vec<(c::Lang, String, c::Count)> = vec![];
        loop {
            match self.chan.steal() {
                // What causes these?
                Stolen::Empty | Stolen::Abort => continue,
                Stolen::Data(Work::Quit) => break,
                Stolen::Data(Work::File(path)) => {
                    let lang = c::lang_from_ext(&path);
                    if lang != c::Lang::Unrecognized {
                        let count = c::count(&path);
                        v.push((lang, String::from(path), count));
                    }
                }
            };
        }
        v
    }
}

fn main() {

    let matches = App::new("count")
        .global_settings(&[AppSettings::ColoredHelp])
        .version("0.1")
        .author("Curtis Gagliardi <curtis@curtis.io>")
        .about("counts things quickly hopefully")
        // TODO(cgag): actually implement filtering
        .arg(Arg::with_name("exclude")
            .required(false)
            .long("exclude")
            .value_name("REGEX")
            .takes_value(true)
            .help("comma separated list of files/directories to ignore"))
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
        .arg(Arg::with_name("target")
            .required(true)
            .help("File or directory to count"))
        .get_matches();

    let filepaths = matches.values_of("target").unwrap();
    let sort = matches.value_of("sort").unwrap_or("language");
    let by_file = matches.is_present("files");
    let exclude_regex = match matches.value_of("exclude") {
        Some(rx_str) => {
            match Regex::new(rx_str) {
                Ok(r) => Some(r),
                Err(e) => {
                    println!("e: {}", e);
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

    for filepath in filepaths {
        if exclude_regex.is_some() {
            let ref r = &exclude_regex.clone().unwrap();
            for entry in WalkDir::new(filepath).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    let path = entry.path().to_str().unwrap();
                    if !r.is_match(path) {
                        workq.push(Work::File(String::from(path)));
                    }
                }
            }
        } else {
            for entry in WalkDir::new(filepath).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    let path = entry.path().to_str().unwrap();
                    workq.push(Work::File(String::from(path)));
                }
            }
        }
    }

    for _ in 0..workers.len() {
        workq.push(Work::Quit);
    }

    let mut counts: Vec<(c::Lang, String, c::Count)> = Vec::new();
    for worker in workers {
        counts.extend(worker.join().unwrap().iter().cloned())
    }

    let mut lang_counts_by_file: HashMap<c::Lang, Vec<(String, c::Count)>> = HashMap::new();
    for (lang, filepath, count) in counts {
        match lang_counts_by_file.entry(lang) {
            Entry::Occupied(mut lang_counts_by_file) => {
                lang_counts_by_file.get_mut().push((filepath, count))
            }
            Entry::Vacant(lang_counts_by_file) => {
                lang_counts_by_file.insert(vec![(filepath, count)]);
            }
        };
    }

    let linesep = "---------------------------------------------------------------------------------";

    if by_file {
        println!("{}", linesep);
        println!(" {0: <18} {1: >8} {2: >12} {3: >12} {4: >12} {5: >12}",
                 "Language",
                 "Files",
                 "Lines",
                 "Blank",
                 "Comment",
                 "Code");
        println!("{}", linesep);

        for (lang, count_vec) in lang_counts_by_file {
            let mut total = c::Count::default();
            for &(_, ref count) in &count_vec {
                total.merge(count);
            }
            println!("{}", linesep);
            println!(" {0: <18} {1: >8} {2: >12} {3: >12} {4: >12} {5: >12}",
                     // lang,
                     lang,
                     count_vec.len(),
                     total.lines,
                     total.blank,
                     total.comment,
                     total.code);
            println!("{}", lang);
            println!("{}", linesep);
            for (path, count) in count_vec {
                let mut path_tail = String::from(path);
                if path_tail.len() > 25 {
                    path_tail = path_tail.chars().skip(path_tail.len() - 25).collect::<String>();
                }
                println!("|{0: <25} {1: >12} {2: >12} {3: >12} {4: >12}",
                         path_tail,
                         count.lines,
                         count.blank,
                         count.comment,
                         count.code);
            }
        }
    } else {
        let mut lang_totals: HashMap<&c::Lang, c::LangTotal> = HashMap::new();
        for (lang, count_vec) in &lang_counts_by_file {
            let mut lang_total: c::Count = Default::default();
            for &(_, ref count) in count_vec {
                lang_total.merge(count);
            }
            lang_totals.insert(lang,
                               c::LangTotal {
                                   files: count_vec.len() as u32,
                                   count: lang_total,
                               });
        }

        let mut totals_by_lang = lang_totals.iter().collect::<Vec<(&&c::Lang, &c::LangTotal)>>();
        match sort {
            "language" => totals_by_lang.sort_by(|&(l1, _), &(l2, _)| l1.to_s().cmp(&l2.to_s())),
            "files" => totals_by_lang.sort_by(|&(_, c1), &(_, c2)| c2.files.cmp(&c1.files)),
            "code" => {
                totals_by_lang.sort_by(|&(_, c1), &(_, c2)| c2.count.code.cmp(&c1.count.code))
            }
            "comment" => {
                totals_by_lang.sort_by(|&(_, c1), &(_, c2)| c2.count.comment.cmp(&c1.count.comment))
            }
            "blank" => {
                totals_by_lang.sort_by(|&(_, c1), &(_, c2)| c2.count.blank.cmp(&c1.count.blank))
            }
            "lines" => {
                totals_by_lang.sort_by(|&(_, c1), &(_, c2)| c2.count.lines.cmp(&c1.count.lines))
            }
            _ => {
                println!("invalid sort option {}, sorting by language", sort);
                totals_by_lang.sort_by(|&(l1, _), &(l2, _)| l1.to_s().cmp(&l2.to_s()))
            }
        }

        let mut totals = c::LangTotal {
            files: 0,
            count: Default::default(),
        };
        for &(_, total) in &totals_by_lang {
            totals.files += total.files;
            totals.count.code += total.count.code;
            totals.count.blank += total.count.blank;
            totals.count.comment += total.count.comment;
            totals.count.lines += total.count.lines;
        }


        println!("{}", linesep);
        println!(" {0: <18} {1: >8} {2: >12} {3: >12} {4: >12} {5: >12}",
                 "Language",
                 "Files",
                 "Lines",
                 "Blank",
                 "Comment",
                 "Code");
        println!("{}", linesep);

        for &(lang, total) in &totals_by_lang {
            println!(" {0: <18} {1: >8} {2: >12} {3: >12} {4: >12} {5: >12}",
                     lang,
                     total.files,
                     total.count.lines,
                     total.count.blank,
                     total.count.comment,
                     total.count.code);
        }

        println!("{}", linesep);
        println!(" {0: <18} {1: >8} {2: >12} {3: >12} {4: >12} {5: >12}",
                 "Total",
                 totals.files,
                 totals.count.lines,
                 totals.count.blank,
                 totals.count.comment,
                 totals.count.code);
        println!("{}", linesep);
    }
}
