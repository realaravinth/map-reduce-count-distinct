use std::sync::mpsc::channel;
use std::thread;

use slog::info;

use crate::map::Map;
use crate::threads::ThreadPool;
use crate::utils::*;
use crate::*;

pub fn runner() {
    let parallel = parallel::ParallelRunner::new();
    info!(LOG, "Running threadpool");
    let start = time::Instant::now();
    parallel.count_pool();
    print_time(start);

    info!(LOG, "Running regular threads");
    let start = time::Instant::now();
    parallel.count();
    print_time(start);
}

pub struct ParallelRunner {
    pub quota: usize,
    pub pos: Vec<usize>,
    pub pool: ThreadPool,
}

impl ParallelRunner {
    pub fn new() -> ParallelRunner {
        let lines = TEXT.lines().count();
        let quota = lines / *NUM_CPU;

        info!(LOG, "Total Lines: {}", lines);
        info!(LOG, "Lines per thread: {}", quota);

        let pos = Self::calc_quota(quota);
        let pool = ThreadPool::new(*NUM_CPU + 1);

        ParallelRunner { quota, pos, pool }
    }

    fn calc_quota(quota: usize) -> Vec<usize> {
        //let mut word_iter = TEXT.split('\n');
        let mut word_iter = TEXT.lines();
        let mut len = 0;

        let mut arr = Vec::new();

        for _ in 0..*NUM_CPU {
            for _ in 0..quota {
                let word = word_iter.next().unwrap();
                len += word.len();
            }
            arr.push(len);
        }

        arr
    }

    pub fn count_pool(&self) {
        let mut map = Map::new();

        for (i, worker) in self.pool.workers.iter().enumerate() {
            let start;
            let end;
            let sub;
            if i == 0 {
                start = 0;
                end = *self.pos.get(i).unwrap();
                sub = &TEXT[start..end];
            } else if i == *NUM_CPU {
                start = *self.pos.get(i - 1).unwrap();
                sub = &TEXT[start..];
            } else {
                start = *self.pos.get(i - 1).unwrap();
                end = *self.pos.get(i).unwrap();
                sub = &TEXT[start..end];
            }

            if worker.tx.is_some() {
                worker.tx.as_ref().unwrap().send(sub).unwrap();
            }
        }

        //let mut count = 0;
        let mut iter = 0;

        while iter < *NUM_CPU + 1 {
            if let Ok((_, wordmap)) = self.pool.rx.recv() {
                map.map = Map::merge(map.map, wordmap.map);
                iter += 1;
            } else {
                break;
            }
        }

        //        info!(LOG, "total words counted: {}", count);

        if CONFIG.display {
            map.display();
        }
    }

    pub fn count(&self) {
        let mut children = Vec::default();
        let mut map = Map::new();

        let (tx, rx) = channel();

        for i in 0..*NUM_CPU + 1 {
            let tx = tx.clone();

            let start;
            let end;
            let sub;
            if i == 0 {
                start = 0;
                end = *self.pos.get(i).unwrap();
                sub = &TEXT[start..end];
            } else if i == *NUM_CPU {
                start = *self.pos.get(i - 1).unwrap();
                sub = &TEXT[start..];
            } else {
                start = *self.pos.get(i - 1).unwrap();
                end = *self.pos.get(i).unwrap();
                sub = &TEXT[start..end];
            }

            let child = thread::spawn(move || {
                let mut map = Map::new();

                let word_iter = sub.split(' ');

                let mut count = 0;

                for word in word_iter {
                    if !word.is_empty() {
                        map.insert(word);
                    }
                    count += 1;
                }

                tx.send((count, map)).unwrap();
                drop(tx);
            });

            children.push(child);
        }
        drop(tx);

        let mut count = 0;

        loop {
            if let Ok((c, wordmap)) = rx.recv() {
                map.map = Map::merge(map.map, wordmap.map);
                count += c;
            } else {
                break;
            }
        }

        children.drain(0..).for_each(|child| child.join().unwrap());

        info!(LOG, "total words counted: {}", count);

        if CONFIG.display {
            map.display();
        }
    }
}
