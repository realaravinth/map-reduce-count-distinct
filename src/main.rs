//use std::time;

use lazy_static::initialize;
use lazy_static::lazy_static;
use slog::info;
use slog::Logger;

mod cli;
mod config;
mod dash;
mod map;
mod parallel;
mod pool;
mod serial;
mod threads;
mod utils;

use config::Method;

//pub const TEXT: &str = include_str!("../res/large.txt");

//#[global_allocator]
//static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

lazy_static! {
    pub static ref TEXT: String = utils::prepare_content();
    pub static ref LOG: Logger = utils::int_logging();
    pub static ref NUM_CPU: usize = 5;
    pub static ref CONFIG: config::Config = cli::cli();
}

fn main() {
    initialize(&TEXT);
    initialize(&CONFIG);
    initialize(&LOG);
    initialize(&NUM_CPU);

    match CONFIG.method {
        Method::Serial => serial::runner(),
        Method::Parallel => parallel::runner(),
        Method::ParallelThreadPool => pool::runner(),
        Method::DashMap => dash::runner(),
    }
}
