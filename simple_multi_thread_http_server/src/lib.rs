use std::thread::{self, JoinHandle};

pub struct Worker {
    id: usize,
    handler: JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            handler: thread::spawn(|| {}),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        Self { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
