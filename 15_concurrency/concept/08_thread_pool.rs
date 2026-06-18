// 08_thread_pool.rs — Basic thread pool, scoped threads
//
// The one question: "How do you run code in parallel — safely?"
// Answer: A thread pool reuses a fixed set of worker threads, avoiding the
// overhead of spawning a new OS thread per task. With scoped threads
// (std::thread::scope in Rust 1.63+), you can borrow data without Arc overhead.

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // --- Scoped threads (Rust 1.63+) ---
    // thread::scope lets you spawn threads that borrow from the parent scope.
    // No need for Arc — scoped threads can borrow references to local variables.
    println!("=== Scoped threads ===");
    scoped_thread_example();

    // --- Simple thread pool ---
    println!("\n=== Thread pool ===");
    thread_pool_example();

    // --- Using the thread pool ---
    println!("\n=== Using the thread pool ===");
    thread_pool_usage_example();

    println!("\nAll thread pool examples done!");
}

// --- Scoped threads: borrow data without Arc ---
fn scoped_thread_example() {
    let numbers = vec![10, 20, 30, 40, 50];

    // thread::scope ensures ALL spawned threads finish before the scope ends.
    // Inside, spawned threads can borrow local variables — no 'static required!
    // The scope exits only after all spawned threads finish.
    //
    // We use an outer Mutex (no Arc needed!) and share a & reference to it.
    let sum = Mutex::new(0u64);
    let sum_ref = &sum;
    let numbers_ref = &numbers;

    thread::scope(|s| {
        // Spawn 5 threads, each adding one element
        s.spawn(|| {
            *sum_ref.lock().unwrap() += numbers_ref[0];
        });
        s.spawn(|| {
            *sum_ref.lock().unwrap() += numbers_ref[1];
        });
        s.spawn(|| {
            *sum_ref.lock().unwrap() += numbers_ref[2];
        });
        s.spawn(|| {
            *sum_ref.lock().unwrap() += numbers_ref[3];
        });
        s.spawn(|| {
            *sum_ref.lock().unwrap() += numbers_ref[4];
        });
        // All spawned threads must finish before this closure returns.
    });

    // After the scope, `numbers` is accessible again.
    println!("Scoped sum (via &ref): {}", *sum.lock().unwrap());
    println!("Numbers still available: {numbers:?}");

    // Alternative: use thread::scope with indices
    let results = Mutex::new(Vec::new());
    let results_ref = &results;

    thread::scope(|s| {
        for i in 0..3 {
            s.spawn(move || {
                let mut r = results_ref.lock().unwrap();
                r.push((i + 1) * 2);
            });
        }
    });

    println!("Scoped results: {:?}", *results.lock().unwrap());
}

// --- Simple Thread Pool implementation ---
// A thread pool reuses worker threads. Each worker pulls a job from a shared
// queue, executes it, then waits for the next.

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let job = {
                let lock = receiver.lock().unwrap();
                lock.recv()
            };

            match job {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    // Sender dropped — shut down this worker
                    break;
                }
            }
        });

        Worker {
            id,
            handle: Some(handle),
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool with `size` worker threads.
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Execute a closure on the thread pool.
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Drop the sender so workers' recv() returns Err, causing them to exit
        drop(self.sender.take());

        // Wait for all workers to finish
        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

// --- Example usage of the thread pool ---
fn thread_pool_example() {
    let pool = ThreadPool::new(4);

    for i in 0..8 {
        pool.execute(move || {
            println!("Job {i} running in thread {:?}", thread::current().id());
            thread::sleep(Duration::from_millis(10));
        });
    }
    println!("Thread pool created and jobs submitted. Pool will join on drop.");
}

fn thread_pool_usage_example() {
    let pool = ThreadPool::new(4);
    let result = Arc::new(Mutex::new(0u64));
    let num_jobs = 12;

    for i in 0..num_jobs {
        let result = Arc::clone(&result);
        pool.execute(move || {
            thread::sleep(Duration::from_millis(5));
            let mut guard = result.lock().unwrap();
            *guard += i as u64;
        });
    }

    drop(pool);
    println!("Sum of indices 0..{num_jobs} = {}", *result.lock().unwrap());
}
