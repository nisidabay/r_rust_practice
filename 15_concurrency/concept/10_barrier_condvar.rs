// 10_barrier_condvar.rs — Barrier and Condvar for thread synchronization
//
// The one question: "How do you run code in parallel — safely?"
// Answer: Use Barrier to synchronize threads at a point, and Condvar to
// notify threads when a condition becomes true.
//
// Barrier: All threads wait at a barrier until N threads have arrived, then
//          all proceed simultaneously.
// Condvar: A condition variable that blocks a thread until notified by another
//          thread. Always paired with a Mutex to check the condition.

use std::sync::{Arc, Barrier, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // --- Barrier: synchronize at a common point ---
    println!("=== Barrier example ===");
    barrier_example();

    // --- Condvar: wait for a condition ---
    println!("\n=== Condvar example ===");
    condvar_example();

    // --- Condvar with timeout ---
    println!("\n=== Condvar with timeout ===");
    condvar_timeout_example();

    // --- Producer-consumer with Condvar ---
    println!("\n=== Producer-consumer with Condvar ===");
    producer_consumer_condvar();

    println!("\nAll synchronization examples done!");
}

fn barrier_example() {
    let num_threads = 5;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];

    for id in 0..num_threads {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            // Phase 1: work individually
            println!("Thread {id}: starting phase 1 work");
            thread::sleep(Duration::from_millis(id as u64 * 10)); // varied work
            println!("Thread {id}: phase 1 done, waiting at barrier");

            // Wait for all threads to reach the barrier
            b.wait();

            // Phase 2: all threads proceed together
            println!("Thread {id}: barrier passed, starting phase 2");
            thread::sleep(Duration::from_millis(5));
            println!("Thread {id}: phase 2 done");
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}

fn condvar_example() {
    // Condvar is always paired with a Mutex
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    let waiter = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut ready = lock.lock().unwrap();

        // Wait until ready is true.
        // while_loop is necessary due to spurious wakeups:
        // wait() atomically releases the mutex and blocks, then re-acquires on wake.
        while !*ready {
            println!("Waiter: waiting for notification...");
            ready = cvar.wait(ready).unwrap();
        }
        println!("Waiter: notified! Condition is true, proceeding.");
    });

    // Give the waiter time to start waiting
    thread::sleep(Duration::from_millis(20));

    let (lock, cvar) = &*pair;
    {
        let mut ready = lock.lock().unwrap();
        *ready = true;
        println!("Notifier: condition set to true");
    }
    // notify_one wakes up one waiting thread
    cvar.notify_one();

    waiter.join().unwrap();
}

fn condvar_timeout_example() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    let waiter = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let ready = lock.lock().unwrap();

        // wait_timeout blocks for at most the given duration
        let (guard, result) = cvar.wait_timeout(ready, Duration::from_millis(50)).unwrap();
        match result.timed_out() {
            true => println!("Waiter: timed out waiting (expected — no one notified)"),
            false => println!("Waiter: notified before timeout"),
        }
        // `guard` is the mutex lock, re-acquired after wait
        drop(guard);
    });

    // Don't notify — let it time out
    waiter.join().unwrap();
}

fn producer_consumer_condvar() {
    // A shared queue protected by Mutex, with Condvar for signaling
    struct SharedQueue {
        items: Vec<u32>,
    }

    let queue = Arc::new((Mutex::new(SharedQueue { items: vec![] }), Condvar::new()));

    let mut handles = vec![];

    // Producer thread
    let q = Arc::clone(&queue);
    handles.push(thread::spawn(move || {
        for i in 0..5 {
            let (lock, cvar) = &*q;
            let mut guard = lock.lock().unwrap();
            guard.items.push(i);
            println!("Producer: produced {i}");
            // Notify consumer that an item is available
            cvar.notify_one();
            // Drop lock before sleeping so consumer can run
            drop(guard);
            thread::sleep(Duration::from_millis(15));
        }
    }));

    // Consumer thread
    let q = Arc::clone(&queue);
    handles.push(thread::spawn(move || {
        let (lock, cvar) = &*q;

        loop {
            let mut guard = lock.lock().unwrap();

            // Wait until there's an item or all producers are done
            while guard.items.is_empty() {
                guard = cvar.wait(guard).unwrap();
            }

            let item = guard.items.remove(0);
            println!("Consumer: consumed {item}");

            if item == 4 {
                // Last item
                println!("Consumer: all items consumed, exiting");
                break;
            }

            // Lock is released when guard is dropped (end of scope)
        }
    }));

    for h in handles {
        h.join().unwrap();
    }

    println!("Producer-consumer completed");
}
