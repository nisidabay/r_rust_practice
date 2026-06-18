// 05_mutex.rs — Arc<Mutex<T>>, lock(), poisoning
//
// The one question: "How do you run code in parallel — safely?"
// Answer: Use Arc<Mutex<T>> to share and mutate data across threads safely.
//
// Mutex = Mutual Exclusion. Only one thread can hold the lock at a time.
// Arc = Atomic Reference Counting. Allows multiple owners of the same data.
// Together: Arc<Mutex<T>> is the standard way to share mutable state.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // --- Basic Mutex usage (single-threaded) ---
    let m = Mutex::new(5);
    {
        // lock() returns MutexGuard<T>, which derefs to &mut T
        let mut guard = m.lock().unwrap();
        *guard += 1;
        // guard is dropped here, releasing the lock
    }
    println!("Mutex value: {:?}", m.lock().unwrap());

    // --- Arc<Mutex<T>> for shared state across threads ---
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut guard = counter.lock().unwrap();
            *guard += 1;
            // Lock is released when `guard` goes out of scope
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());

    // --- Locking blocks the thread ---
    // If a thread tries to lock a Mutex that's already locked, it blocks
    // (spins/sleeps) until the lock is released.
    let pair = Arc::new(Mutex::new((0u32, 0u32)));
    let pair2 = Arc::clone(&pair);

    let h1 = thread::spawn(move || {
        let mut guard = pair2.lock().unwrap();
        guard.0 = 42;
        thread::sleep(std::time::Duration::from_millis(20));
        guard.1 = 99;
        println!("Thread 1 done, pair = ({}, {})", guard.0, guard.1);
    });

    let h2 = thread::spawn(move || {
        // This thread blocks until thread 1 releases the lock (20ms later)
        let guard = pair.lock().unwrap();
        println!("Thread 2 sees pair = ({}, {})", guard.0, guard.1);
    });

    h1.join().unwrap();
    h2.join().unwrap();

    // --- Poisoning ---
    // If a thread panics while holding a Mutex lock, the Mutex becomes poisoned.
    // Subsequent lock() calls will return Err(PoisonError).
    let poisoned_mutex = Arc::new(Mutex::new(String::from("hello")));
    let pm2 = Arc::clone(&poisoned_mutex);

    let poison_handle = thread::spawn(move || {
        let _guard = pm2.lock().unwrap();
        panic!("Intentional panic while holding the lock!");
    });

    // Ignore the panic
    let _ = poison_handle.join();

    // The mutex is now poisoned
    let result = poisoned_mutex.lock();
    match result {
        Ok(guard) => println!("Got lock: {guard}"),
        Err(poisoned) => {
            // You can recover the value inside a PoisonError
            let guard = poisoned.into_inner();
            println!("Mutex is poisoned! Inner value: {guard}");
        }
    }

    println!("All Mutex examples done!");
}
