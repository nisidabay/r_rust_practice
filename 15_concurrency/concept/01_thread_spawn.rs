// 01_thread_spawn.rs — thread::spawn, join, basic threading
//
// The one question: "How do you run code in parallel — safely?"
// Answer: Start with thread::spawn and join handles.
//
// thread::spawn creates a new OS thread that runs the closure.
// The returned JoinHandle lets you wait for the thread to finish via .join().
// If you drop the handle without joining, the thread still runs, but you can't
// get its result or know when it finishes.

use std::thread;
use std::time::Duration;

fn main() {
    // --- Spawning a thread ---
    // thread::spawn takes a closure. The closure runs in a new OS thread.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread: count {i}");
            thread::sleep(Duration::from_millis(1)); // tiny delay
        }
    });

    // Meanwhile, the main thread keeps running.
    for i in 1..5 {
        println!("Main thread: count {i}");
        thread::sleep(Duration::from_millis(1));
    }

    // --- Joining ---
    // .join() blocks the current thread until the spawned thread finishes.
    // It returns Result<T, Box<dyn Any + Send>> — Err if the thread panicked.
    println!("Waiting for spawned thread to finish...");
    handle.join().expect("Spawned thread panicked!");
    println!("Both threads done.");

    // --- Returning a value from a thread ---
    // The closure can return a value, which .join() gives back.
    let compute_handle = thread::spawn(|| {
        let sum: u64 = (1..=100).sum();
        sum // return value from closure
    });

    let result = compute_handle.join().expect("Compute thread panicked!");
    println!("Sum 1..100 from thread: {result}");

    // --- Multiple threads ---
    let mut handles = vec![];
    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Worker {i} starting work");
            thread::sleep(Duration::from_millis(10));
            println!("Worker {i} done");
            i * 10 // return the index times 10
        });
        handles.push(handle);
    }

    // Collect results from all threads
    let results: Vec<i32> = handles
        .into_iter()
        .map(|h| h.join().expect("Thread panicked"))
        .collect();
    println!("Results from workers: {results:?}");

    // --- Thread name and stack size (builder) ---
    let builder = thread::Builder::new()
        .name("my-custom-thread".into())
        .stack_size(64 * 1024); // 64 KB stack

    let named_handle = builder
        .spawn(|| {
            println!(
                "Running in thread: {:?}",
                thread::current().name()
            );
        })
        .expect("Failed to spawn named thread");

    named_handle.join().expect("Named thread panicked");
    println!("All done!");
}
