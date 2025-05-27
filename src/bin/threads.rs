use std::sync::{Arc, Mutex,mpsc};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

//Crea una nueva ThreadPool
//El tamaño es el número de threads en el pool
//Tener 0 o threads negativos no hace el menor sentido
impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        //Crear threads que alamcenamos en el vector
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {workers, sender}
    }
}

//Cada thread necesita un ID
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move|| {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {id} se puso a chambear; ejecutándose.");

                job();
            }
        });
        Worker { id, thread }
    }
}