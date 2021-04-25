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
    pub static ref NUM_CPU: usize = num_cpus::get() - 2;
}

fn main() {
    initialize(&TEXT);

    //    info!(LOG, "Counting words searially");
    //    serial::serial();
    //
    //    utils::section_break();
    //
    info!(LOG, "Counting words parallelly");

    let parallel = parallel::ParallelRunner::new();
    let start = time::Instant::now();
    parallel.run();
    println!("Time elapsed:");
    println!("  {} micro seconds", start.elapsed().as_micros());
    println!("  {} nano seconds", start.elapsed().as_nanos());
    println!("  {} seconds", start.elapsed().as_secs_f64());
}
