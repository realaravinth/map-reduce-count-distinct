use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};

use crate::map::Map;

pub struct Worker {
    pub join_handle: JoinHandle<()>,
    pub tx: Option<Sender<&'static str>>,
    pub pool_tx: Option<Sender<(usize, Map<'static>)>>,
}

impl Worker {
    pub fn new(pool_tx: Sender<(usize, Map<'static>)>) -> Self {
        //-> Self {
        let (tx, rx): (Sender<&str>, Receiver<&str>) = channel();

        let send_pool = pool_tx.clone();
        let join_handle = thread::spawn(move || {
            //    let t=rx.recv();
            loop {
                if let Ok(text) = rx.recv() {
                    let mut map = Map::new();

                    let word_iter = text.split(' ');

                    let mut count = 0;

                    for word in word_iter {
                        if !word.is_empty() {
                            map.insert(word);
                        }
                        count += 1;
                    }

                    send_pool.send((count, map)).unwrap();
                    //                   drop(tx);
                } else {
                    break;
                }
            }
        });

        let tx = Some(tx);
        let pool_tx = Some(pool_tx);

        Worker {
            join_handle,
            tx,
            pool_tx,
        }
    }
}

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub tx: Option<Sender<(usize, Map<'static>)>>,
    pub rx: Receiver<(usize, Map<'static>)>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let mut workers = Vec::new();

        let (tx, rx) = channel();
        for _ in 0..size {
            workers.push(Worker::new(tx.clone()));
        }

        let tx = Some(tx);
        ThreadPool { workers, tx, rx }
    }
}
