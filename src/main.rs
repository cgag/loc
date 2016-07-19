extern crate clap;
extern crate count;

use clap::{Arg, App, AppSettings};

use count::*;

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

    for filepath in matches.values_of("target").unwrap() {
        println!("filepath: {}", filepath);
        println!("count: {:?}", count_regex(filepath));
    }
}
