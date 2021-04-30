use clap::{App, Arg};

use crate::config::*;

pub fn cli() -> Config {
    let matches = App::new("Word Count")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Counts words")
        .arg(
            Arg::with_name("serial")
                .short("-s")
                .long("--serial")
                .value_name("FILE")
                .help("Count searially")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("parallel_threadpool")
                .short("-t")
                .long("--threadpool")
                .help("Count parallelly using threadpool")
                .required(false),
        )
        .arg(
            Arg::with_name("dash")
                .short("-d")
                .long("--dashmap")
                .help("Count parallelly")
                .required(false),
        )
        .arg(
            Arg::with_name("parallel")
                .short("-p")
                .long("--parallel")
                .help("Count parallelly")
                .required(false),
        )
        .arg(
            Arg::with_name("verbose")
                .short("-v")
                .long("--verbose")
                .help("Display word count mapping")
                .required(false),
        )
        .get_matches();

    let display;
    if matches.is_present("verbose") {
        display = true;
    } else {
        display = false;
    }

    let method = {
        if matches.is_present("serial") {
            Method::Serial
        } else if matches.is_present("parallel") {
            Method::Parallel
        } else if matches.is_present("parallel_threadpool") {
            Method::ParallelThreadPool
        } else if matches.is_present("dash") {
            Method::DashMap
        } else {
            panic!("Set method")
        }
    };

    Config { display, method }
}
