use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Shutdown,
}

pub struct ThreadPool {
    sender: mpsc::Sender<Message>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { sender, workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Message::NewJob(Box::new(f))).unwrap();
    }

    pub fn shutdown(mut self) {
        for _ in &self.workers {
            let _ = self.sender.send(Message::Shutdown);
        }
        for w in &mut self.workers {
            if let Some(handle) = w.handle.take() {
                let _ = handle.join();
            }
        }
    }
}

struct Worker {
    #[allow(dead_code)]
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let msg = receiver.lock().unwrap().recv();
            match msg {
                Ok(Message::NewJob(job)) => job(),
                Ok(Message::Shutdown) | Err(_) => break,
            }
        });

        Self { id, handle: Some(handle) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn executes_jobs() {
        let pool = ThreadPool::new(4);
        let out = Arc::new(Mutex::new(Vec::new()));

        for i in 0..20 {
            let out = Arc::clone(&out);
            pool.execute(move || out.lock().unwrap().push(i));
        }

        pool.shutdown();
        assert_eq!(out.lock().unwrap().len(), 20);
    }
}
