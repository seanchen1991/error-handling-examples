use std::thread;
use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn FnOnce() + 'static + Send>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock()
                .unwrap()
                .recv()
                .unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();      
                },
                Message::Terminate => {
                    println!("Worker {} received message to terminate.", id);
                    break;
                },
            }
        });

        Worker { 
            id, 
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending `Terminate` message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap(); 
        }
        
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>, 
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> Self {
        assert!(num_threads > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num_threads);

        for id in 0..num_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { 
            workers,
            sender,
        }
    } 

    pub fn execute<C>(&self, closure: C) 
    where 
        C: FnOnce() + Send + 'static
    {
        let job = Box::new(closure);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}