use crate::map::Map;
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
    info!(LOG, "Counting words searially");
    serial();
}
