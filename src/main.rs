#![feature(iter_advance_by)]

use std::time;

use lazy_static::initialize;
use lazy_static::lazy_static;
use slog::info;
use slog::Logger;

mod map;
mod parallel;
mod serial;
mod utils;

//pub const TEXT: &str = include_str!("../res/large.txt");

lazy_static! {
    pub static ref TEXT: String = utils::prepare_content();
    pub static ref LOG: Logger = utils::int_logging();
    pub static ref NUM_CPU: usize = num_cpus::get();
}

fn main() {
    initialize(&TEXT);

    info!(LOG, "Counting words");
    serial::serial();

    info!(LOG, "exit");
    let parallel = parallel::ParallelRunner::new();
    parallel.run();
}
