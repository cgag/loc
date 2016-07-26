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
    let mut counts: Vec<(c::Lang, &str, c::Count)> = Vec::new();
    let mut files_processed = 0;
    for filepath in filepaths {
        for entry in WalkDir::new(filepath) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                let p = entry.path().to_str().unwrap();
                let lang = c::lang_from_ext(p);
                if lang != c::Lang::Unrecognized {
                    files_processed += 1;
                    let count = c::count_mmap_unsafe(&p, &c::counter_config_for_lang(&lang));
                    counts.push((lang, filepath, count));
                }
            }
        }
    }

    let mut lang_counts: HashMap<c::Lang, c::Count> = HashMap::new();
    for &(ref lang, _, ref count) in &counts {
        match lang_counts.entry(*lang) {
            Entry::Occupied(mut lang_count) => lang_count.get_mut().merge(&count),
            Entry::Vacant(lang_count) => {
                let new_count: c::Count = Default::default();
                lang_count.insert(new_count);
            }
        };
    }

    for (k, v) in lang_counts {
        println!("k: {:?}, v: {:?}", k, v);
    }

    println!("files processed {}", files_processed);

    // for (group_lang, group) in counts.iter().group_by(|&&(ref lang, _, _)| lang) {
    //     let mut total_count: c::Count = Default::default();
    //     for &(_, _, ref count) in group {
    //         total_count.merge(count);
    //     }
    //     println!("Total count: {:?}", total_count);
    // }

}
