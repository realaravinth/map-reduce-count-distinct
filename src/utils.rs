use std::fs;
use std::path::Path;
use std::time;

use slog::info;
use slog::o;
use slog::Drain;
use slog::Logger;

use crate::*;

pub fn prepare_content() -> String {
    const UNWANTED_CHAR: [char; 23] = [
        ',', ':', '?', '!', ';', '.', '[', ']', '(', ')', '^', '*', '$', '%', '#', '&', '{', '}',
        '`', '~', '\t', '—', '_',
    ];

    const FILE: &str = "./res/large.txt";
    const PROCESSED_FILE: &str = "./res/large-processed.txt";

    if Path::new(PROCESSED_FILE).exists() {
        fs::read_to_string(PROCESSED_FILE).unwrap()
    } else {
        info!(LOG, "Reading File");
        let text = fs::read_to_string(FILE).unwrap();
        info!(LOG, "Processing File");

        let start = time::Instant::now();

        let processed_text: String = text
            .chars()
            .map(|character| {
                if UNWANTED_CHAR.contains(&character) {
                    ' '
                } else {
                    character
                }
            })
            .collect();

        info!(
            LOG,
            "Finished processing in {} seconds",
            start.elapsed().as_secs_f64()
        );

        fs::write(PROCESSED_FILE, &processed_text).unwrap();

        processed_text
    }
}

pub fn int_logging() -> Logger {
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let async_drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(async_drain, o!("version" => "0.1.0"))
}

pub fn print_time(instant: time::Instant) {
    println!("Time elapsed:");
    println!("  {} micro seconds", instant.elapsed().as_micros());
    println!("  {} nano seconds", instant.elapsed().as_nanos());
    println!("  {} seconds", instant.elapsed().as_secs_f64());
}
