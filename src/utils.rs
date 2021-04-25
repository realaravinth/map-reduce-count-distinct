use std::fs;

use slog::info;
use slog::o;
use slog::Drain;
use slog::Logger;

use crate::*;

pub fn prepare_content() -> String {
    const UNWANTED_CHAR: [char; 26] = [
        ',', ':', '?', '!', ';', '\r', '\'', '.', '[', ']', '(', ')', '^', '*', '$', '%', '#', '&',
        '{', '}', '`', '~', '\n', '\t', '—', '_',
    ];

    info!(LOG, "Reading File");
    let text = fs::read_to_string("./res/large.txt").unwrap();
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

    processed_text
}

pub fn int_logging() -> Logger {
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let async_drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(async_drain, o!("version" => "0.1.0"))
}
