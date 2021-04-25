use std::time;

use lazy_static::initialize;
use lazy_static::lazy_static;
use slog::info;
use slog::Logger;

mod cli;
mod config;
mod map;
mod parallel;
mod serial;
mod utils;

use config::Method;

//pub const TEXT: &str = include_str!("../res/large.txt");

lazy_static! {
    pub static ref TEXT: String = utils::prepare_content();
    pub static ref LOG: Logger = utils::int_logging();
    pub static ref NUM_CPU: usize = num_cpus::get() - 2;
    pub static ref CONFIG: config::Config = cli::cli();
}

fn main() {
    initialize(&TEXT);

    match CONFIG.method {
        Method::Serial => serial::runner(),

        Method::Parallel => parallel::runner(),
    }
}
