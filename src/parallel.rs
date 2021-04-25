use std::sync::mpsc::channel;
use std::thread;

use slog::info;

use crate::map::Map;
use crate::*;

#[derive(Debug, Clone)]
pub struct ParallelRunner {
    pub quota: usize,
}

impl ParallelRunner {
    pub fn new() -> ParallelRunner {
        let quota = TEXT.split(' ').count() / *NUM_CPU;
        info!(LOG, "Words per thread: {}", quota);
        ParallelRunner { quota }
    }

    pub fn run(&self) {
        let mut children = Vec::default();
        let mut word_iter = TEXT.split(' ');
        let mut map = Map::new();

        let (tx, rx) = channel();

        for i in 0..*NUM_CPU {
            Self::advance_by_quota(self.quota, &mut word_iter);

            let quota = self.quota;

            let tx = tx.clone();

            let child = thread::spawn(move || {
                let mut map = Map::new();

                let mut word_iter = TEXT.split(' ');
                for _ in 0..(quota * i) {
                    word_iter.next();
                }

                for _ in 0..quota {
                    let word = word_iter.next().unwrap();
                    if !word.is_empty() {
                        map.insert(word);
                    }
                }

                tx.send(map).unwrap();
                drop(tx);
            });

            children.push(child);
        }
        drop(tx);

        loop {
            if let Ok(count) = rx.recv() {
                map.map = Map::merge(map.map, count.map);
            } else {
                break;
            }
        }

        children.drain(0..).for_each(|child| child.join().unwrap());

        if CONFIG.display {
            map.display();
        }
    }

    #[inline]
    fn advance_by_quota(quota: usize, iter: &mut std::str::Split<'_, char>) {
        for _ in 0..quota {
            iter.next();
        }
    }
}

pub fn runner() {
    let parallel = parallel::ParallelRunner::new();
    let start = time::Instant::now();
    parallel.run();

    println!("Time elapsed:");
    println!("  {} micro seconds", start.elapsed().as_micros());
    println!("  {} nano seconds", start.elapsed().as_nanos());
    println!("  {} seconds", start.elapsed().as_secs_f64());
}
