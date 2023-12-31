use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Define a struct for the thread pool
pub struct ThreadPool {
    workers: Vec<Worker>,          // Collection of worker threads
    sender: mpsc::Sender<Message>, // Channel sender for sending jobs to workers
}

// Define a struct for the worker thread
struct Worker {
    id: usize,                              // Worker ID
    thread: Option<thread::JoinHandle<()>>, // Handle to the worker thread
}

enum Message {
    NewJob(Job),
    Terminate,
}
// Define a trait for a boxed closure (function)
trait FnBox {
    fn call_box(self: Box<Self>);
}

// Implement the trait for any FnOnce closure
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

// Define the type for a job (boxed closure) that can be sent across threads
type Job = Box<dyn FnBox + Send + 'static>;

// Implement methods for the ThreadPool struct
impl ThreadPool {
    // Create a new thread pool with the specified size
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Create a channel for communication between the main thread and workers
        let (sender, receiver) = mpsc::channel();

        // Wrap the receiver in an Arc (atomic reference count) and a Mutex (mutual exclusion)
        let receiver = Arc::new(Mutex::new(receiver));

        // Create a vector to hold the worker threads
        let mut workers = Vec::with_capacity(size);

        // Create the specified number of worker threads
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // Return the thread pool with the workers and sender channel
        ThreadPool { workers, sender }
    }

    // Execute a job in the thread pool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down Worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
// Implement methods for the Worker struct
impl Worker {
    // Create a new worker thread
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // Lock the receiver to receive a job
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        // Print a message and execute the job
                        println!("Worker {} got a job; executing.", id);
                        job.call_box();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    }
                }
            }
        });

        // Return the worker with the ID and thread handle
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
