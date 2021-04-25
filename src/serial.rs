use crate::map::Map;
use crate::*;

fn serial() {
    let mut map = Map::new();

    let iter = TEXT.split(" ");

    let start = time::Instant::now();
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

    println!("Time elapsed:");
    println!("  {} micro seconds", start.elapsed().as_micros());
    println!("  {} nano seconds", start.elapsed().as_nanos());
    println!("  {} seconds", start.elapsed().as_secs_f64());
    println!("Words counted: {}", words);
    println!("Number of unique words: {}", map.map.len());
}

pub fn runner() {
    info!(LOG, "Counting words searially");
    serial();
}
