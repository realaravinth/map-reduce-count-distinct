use std::time;

use crate::map::Map;
use crate::utils::*;
use crate::*;

fn serial() {
    let mut map = Map::new();

    let iter = TEXT.split(" ");

    let mut words = 0;
    iter.for_each(|word| {
        if !word.is_empty() {
            words += 1;
            map.insert(word);
        }
    });

    if CONFIG.display {
        map.display();
    }
}

pub fn runner() {
    let start = time::Instant::now();
    info!(LOG, "Counting words searially");
    serial();
    print_time(start);
}
