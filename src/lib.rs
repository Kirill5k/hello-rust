use std::thread;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let workers = (0..size)
            .into_iter()
            .map(Worker::new)
            .collect();

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
    }
}
