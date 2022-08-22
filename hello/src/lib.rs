#![allow(dead_code)]
use std::{thread, sync::{mpsc, Arc, Mutex}};

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroSizeError(String)
}

#[derive(Debug)]
struct Worker {
    id: usize,
    // thread: thread::JoinHandle<Arc<Mutex<mpsc::Receiver<Job>>>>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
       let thread = thread::spawn(move || loop {
        match receiver.lock().unwrap().recv() {
            Ok(job) => {
                println!("Worker {id} got a job; executing");
                job();
            }, 
            Err(_) => {
                println!("Worker {id} disconnected; shutting down");
                break;
            }
        }
       });

       Worker { id, thread: Some(thread) }
    }
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static,  {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
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
            return Ok(ThreadPool { workers, sender: Some(sender) })
        }

        Err(PoolCreationError::ZeroSizeError(String::from("Thread pool cannot have a size of 0")))
    }
}

///Implementing the Drop trait for ThreadPool
/// Ensures threads complete their job before closing
impl Drop for ThreadPool {
    fn drop(&mut self) {
       for worker in &mut self.workers {
        drop(self.sender.take());
        println!("Shutting down worker {}", worker.id);
        if let Some(thread) = worker.thread.take() {
            thread.join().unwrap();
        }
       }
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