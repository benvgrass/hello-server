use std::{
    sync::{mpsc, Arc, Mutex},
    thread
};
pub struct ThreadPool {
    workers: Vec<WorkerThread>,
    sender: Option<mpsc::Sender<Job>>
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

        Self { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {

        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("shutting down thread {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


struct WorkerThread {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl WorkerThread {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("thread {id} executing new job");
                    job();
                }
                Err(_) => {
                    println!("thread {id} shutting down");
                    break;
                }
            }

        });
        Self { id, thread: Some(thread) }
    }
}

