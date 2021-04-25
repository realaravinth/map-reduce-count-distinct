use std::thread;

use crate::map::Map;
use crate::*;

#[derive(Debug, Clone)]
pub struct ParallelRunner {
    pub each_worker: Vec<(usize, usize)>,
}

impl ParallelRunner {
    pub fn new() -> ParallelRunner {
        let len = TEXT.len();
        let mut each_worker = Vec::default();
        let quota = len / *NUM_CPU;

        for i in 0..*NUM_CPU {
            let start = i * quota;
            let end;
            if i + 1 == *NUM_CPU {
                end = (i + 1) * quota;
            } else {
                end = ((i + 1) * quota) + 1;
            }
            each_worker[i] = (start, end);
        }

        ParallelRunner { each_worker }
    }

    pub fn run(&self) {
        let mut children = Vec::default();

        self.each_worker.iter().for_each(|(start, end)| {
            let start = start.clone();
            let end = end.clone();
            let mut line_iter = TEXT.lines();

            let child = thread::spawn(move || {
                let mut map = Map::new();
                let mut counter = start;
                line_iter.advance_by(start).unwrap();
                for line in line_iter {
                    if counter == end {
                        break;
                    }

                    let iter = line.split(" ");
                    iter.for_each(|word| {
                        if !word.is_empty() {
                            map.insert(word);
                        }
                    });

                    counter += 1;
                }
            });

            children.push(child);
        });

        children.drain(0..).for_each(|child| child.join().unwrap());
    }
}
