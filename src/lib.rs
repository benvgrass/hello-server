use std::{
    sync::{mpsc, Arc, Mutex},
    thread
};
pub struct ThreadPool {
    workers: Vec<WorkerThread>,
    sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(WorkerThread::new(id, receiver.clone()));
        }

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {

        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct WorkerThread {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl WorkerThread {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });
        Self { id, thread }
    }
}

