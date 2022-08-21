#![allow(dead_code)]
use std::{thread, sync::{mpsc, Arc, Mutex}};

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroSizeError(String)
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<Arc<Mutex<mpsc::Receiver<Job>>>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
       let thread = thread::spawn(move || loop {
        let job = receiver.lock().expect("Thread failed to release the mutex").recv().unwrap();
        println!("Worker {id} got a job; executing");
        job();
       });

       Worker { id, thread }
    }
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static,  {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

    /// Creates a new ThreadPool.
    /// size is the number of threads in the pool.
    /// Has error handling as the function returns a result
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let mut workers = Vec::with_capacity(size);
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            for i in 0..size {
                workers.push(Worker::new(i, Arc::clone(&receiver)));
            }
            return Ok(ThreadPool { workers, sender })
        }

        Err(PoolCreationError::ZeroSizeError(String::from("Thread pool cannot have a size of 0")))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_error_handling() {
        let good_result = ThreadPool::build(1);
        let bad_result = ThreadPool::build(0);

        assert!(good_result.is_ok());
        assert!(bad_result.is_err());
    }
}