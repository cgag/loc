extern crate count;

extern crate clap;
extern crate walkdir;
extern crate itertools;
extern crate crossbeam;
extern crate scoped_threadpool;

use scoped_threadpool::Pool;

use clap::{Arg, App, AppSettings};
use walkdir::WalkDir;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::{Arc, Mutex};

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


    let mut pool = Pool::new(4);

    let mut entries: Vec<walkdir::DirEntry> = Vec::new();
    let filepaths = matches.values_of("target").unwrap();
    for filepath in filepaths {
        for entry in WalkDir::new(filepath) {
            entries.push(entry.unwrap());
        }
    }

    let counts: Arc<Mutex<Vec<(c::Lang, String, c::Count)>>> = Arc::new(Mutex::new(Vec::new()));

    pool.scoped(|scope| {
        for entry in &entries {
            let counts = counts.clone();
            scope.execute(move || {
                if entry.file_type().is_file() {
                    let path = entry.path().to_str().unwrap();
                    let lang = c::lang_from_ext(path);
                    if lang != c::Lang::Unrecognized {
                        let mut data = counts.lock().unwrap();
                        let count = c::count(path);
                        data.push((lang, String::from(path), count));
                    }
                }
            });
        }
    });

    let counts = counts.clone();
    let counts = (*counts.lock().unwrap()).clone();

    let mut lang_counts: HashMap<c::Lang, Vec<(String, c::Count)>> = HashMap::new();
    for (lang, filepath, count) in counts {
        match lang_counts.entry(lang) {
            Entry::Occupied(mut lang_counts) => lang_counts.get_mut().push((filepath, count)),
            Entry::Vacant(lang_count) => {
                lang_count.insert(vec![(filepath, count)]);
            }
        };
    }

    // for (ref lang, ref mut count_vec) in &mut lang_counts {
    //     count_vec.sort_by(|&(_, ref c1), &(_, ref c2)| c1.code.cmp(&c2.code).reverse());
    // for &(ref fpath, ref count) in count_vec.iter() {
    //     println!("fpath: {}, lang: {:?}, v: {:?}", fpath, lang, count);
    // }
    // }



    println!("-------------------------------------------------------------------------------");
    println!(" {0: <12} {1: >12} {2: >12} {3: >12} {4: >12} {5: >12}",
             "Language",
             "Files",
             "Lines",
             "Blank",
             "Comment",
             "Code");
    println!("-------------------------------------------------------------------------------");

    for (lang, ref mut count_vec) in &mut lang_counts {
        let mut lang_total: c::Count = Default::default();
        let mut lang_files = 0;
        for &(_, ref count) in count_vec.iter() {
            lang_total.merge(count);
            lang_files += 1;
        }

        println!(" {0: <12} {1: >12} {2: >12} {3: >12} {4: >12} {5: >12}",
                 lang,
                 lang_files,
                 lang_total.lines,
                 lang_total.blank,
                 lang_total.comment,
                 lang_total.code);
    }

    let mut total_count: c::Count = Default::default();
    for count_vec in lang_counts.values() {
        for &(_, ref count) in count_vec {
            total_count.merge(count);
        }
    }

    println!("-------------------------------------------------------------------------------");
    println!(" {0: <12} {1: >12} {2: >12} {3: >12} {4: >12} {5: >12}",
             "Total",
             10,
             20,
             30,
             40,
             50);
    println!("-------------------------------------------------------------------------------");
    // println!("files processed {}", files_processed);

    // for (ref lang, ref mut count_vec) in &mut lang_counts {
    //     for &(ref fpath, ref count) in count_vec.iter() {
    //         if **lang == c::Lang::C {
    // println!("{} {}", fpath, count.blank);
    // }
    // }
    // }
}
