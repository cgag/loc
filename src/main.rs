extern crate count;

extern crate clap;
extern crate walkdir;
extern crate itertools;

use clap::{Arg, App, AppSettings};
use walkdir::WalkDir;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use count as c;

fn main() {

    let matches = App::new("count")
        .global_settings(&[AppSettings::ColoredHelp])
        .version("0.1")
        .author("Curtis Gagliardi <curtis@curtis.io>")
        .about("counts things quickly hopefully")
        .arg(Arg::with_name("exclude")
            .required(false)
            .long("exclude")
            .value_name("FILES/DIRS")
            .takes_value(true)
            .value_delimiter(",")
            .help("comma separated list of files/directories to ignore"))
        .arg(Arg::with_name("target")
            .required(true)
            .help("The file or directory to count lines in/of"))
        .get_matches();

    let filepaths = matches.values_of("target").unwrap();

    let mut counts: Vec<(c::Lang, String, c::Count)> = Vec::new();
    for filepath in filepaths {
        for entry in WalkDir::new(filepath) {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => {
                    continue;
                }
            };

            if entry.file_type().is_file() {
                let path = entry.path().to_str().unwrap();
                let lang = c::lang_from_ext(path);
                if lang != c::Lang::Unrecognized {
                    let count = c::count(path);
                    counts.push((lang, String::from(path), count));
                }
            }
        }
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

    let mut lang_totals: HashMap<&c::Lang, c::LangTotal> = HashMap::new();
    for (lang, count_vec) in &lang_counts_by_file {
        // TODO(cgag): use a fold?
        let mut lang_total: c::Count = Default::default();
        for &(_, ref count) in count_vec.iter() {
            lang_total.merge(count);
        }
        lang_totals.insert(lang,
                           c::LangTotal {
                               files: count_vec.len() as u32,
                               count: lang_total,
                           });
    }

    let mut totals_by_lang = lang_totals.iter().collect::<Vec<(&&c::Lang, &c::LangTotal)>>();
    totals_by_lang.sort_by(|&(_, c1), &(_, c2)| c2.count.code.cmp(&c1.count.code));

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

    let linesep = "---------------------------------------------------------------------------------";

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
