use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};

// Thread pool worker
struct Worker {
    id: i32,
    entity: JoinHandle<()>,
}

impl Worker {
    // Constructor for the Worker struct
    // Parameters:
    // - id: Identifier for the worker
    // - recv: Arc<Mutex<Receiver<Box<dyn FnOnce() + Send>>>>
    //        Receiver to receive functions to execute.
    // Returns: A new instance of Worker
    fn new(id: i32, recv: Arc<Mutex<Receiver<Box<dyn FnOnce() + Send>>>>) -> Worker {
        let entity = 
        thread::spawn(move || {
            loop{
                let func: Box<dyn FnOnce() + Send>;
                {
                    // Lock the receiver to access the channel
                    let result = recv.lock();
                    func = match result {
                        Ok(guard) => {
                            // Try to receive a function from the channel
                            match guard.recv() {
                                Ok(f) => f,
                                Err(_) => break, // Break the loop if the channel is closed
                            }
                        }
                        Err(_) => break, // Break the loop if the lock is poisoned
                    }
                }
                func()
            }
        });
        Worker {
            id,
            entity,
        }
    }
}


// Thread pool
pub struct ThreadPool {
    workers: Vec<Worker>,
    workers_num: i32,
    sender: Sender<Box<dyn FnOnce() -> () + Send + 'static>>
}

impl ThreadPool {
    // Constructor for the ThreadPool struct
    // Parameters:
    // - workers_num: Number of workers in the pool
    // Returns: A new instance of ThreadPool
    pub fn new(workers_num: i32) -> ThreadPool {
        let mut workers: Vec<Worker> = Vec::new();
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..workers_num {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            workers_num,
            sender
        }
    }
    // Execute a function in one of the worker threads of the pool
    // Parameters:
    // - func: A function that implements FnOnce() trait, which is the task to be executed in a worker thread
    // Returns: Result<(), Box<dyn Error>> indicating whether the function was successfully sent to a worker
    pub fn execute<F: FnOnce() -> () + Send + 'static>(&mut self, func: F) -> Result<(), Box<dyn Error>> {
        self.sender.send(Box::new(func))?;
        Ok(())
    }
}
