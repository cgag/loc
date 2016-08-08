extern crate count;

extern crate clap;
extern crate walkdir;
extern crate itertools;

use clap::{Arg, App, AppSettings};
use walkdir::WalkDir;
// use itertools::Itertools;

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
    let mut files_processed = 0;
    for filepath in filepaths {
        for entry in WalkDir::new(filepath) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                let path = entry.path().to_str().unwrap();
                let lang = c::lang_from_ext(path);
                if lang != c::Lang::Unrecognized {
                    files_processed += 1;
                    let count = c::count(path);
                    println!("count: {:?}", count);
                    counts.push((lang, String::from(path), count));
                }
            }
        }
    }

    let mut lang_counts: HashMap<c::Lang, Vec<(String, c::Count)>> = HashMap::new();
    for (lang, filepath, count) in counts {
        match lang_counts.entry(lang) {
            Entry::Occupied(mut lang_counts) => lang_counts.get_mut().push((filepath, count)),
            Entry::Vacant(lang_count) => {
                lang_count.insert(vec![(filepath, count)]);
            }
        };
    }

    for (ref lang, ref mut count_vec) in &mut lang_counts {
        count_vec.sort_by(|&(_, ref c1), &(_, ref c2)| c1.code.cmp(&c2.code).reverse());
        for &(ref fpath, ref count) in count_vec.iter() {
            println!("fpath: {}, lang: {:?}, v: {:?}", fpath, lang, count);
        }
    }

    for (ref lang, ref mut count_vec) in &mut lang_counts {
        let mut lang_total: c::Count = Default::default();
        let mut lang_files = 0;
        for &(_, ref count) in count_vec.iter() {
            lang_total.merge(count);
            lang_files += 1;
        }
        println!("lang: {:?}, files: {}, c: {:?}",
                 lang,
                 lang_files,
                 lang_total);
    }

    let mut total_count: c::Count = Default::default();
    for count_vec in lang_counts.values() {
        for &(_, ref count) in count_vec {
            total_count.merge(count);
        }
    }

    println!("total count: {:?}", total_count);
    println!("files processed {}", files_processed);

    for (ref lang, ref mut count_vec) in &mut lang_counts {
        for &(ref fpath, ref count) in count_vec.iter() {
            if **lang == c::Lang::C {
                println!("{} {}", fpath, count.code);
            }
        }
    }
}
