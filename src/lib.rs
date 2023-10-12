use std::thread;
pub struct ThreadPool {
    threads: Vec<WorkerThread>
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);
        for id in 0..size {
            threads.push(WorkerThread::new(id));
        }

        Self {threads}
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {

    }
}

struct WorkerThread {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl WorkerThread {
    pub fn new(id: usize) -> Self {
        let thread = thread::spawn(|| {});
        Self { id, thread }
    }
}

