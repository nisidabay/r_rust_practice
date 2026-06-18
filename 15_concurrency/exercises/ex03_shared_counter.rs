// ex03_shared_counter.rs — Atomic counter shared across N threads
//
// Task: Create an atomic counter (AtomicU64), spawn N threads, each thread
// increments the counter M times. Verify the final value is N * M.
//
// The one question: "How do you run code in parallel — safely?"
// Answer: Use AtomicU64 with fetch_add for lock-free, thread-safe counting.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let num_threads = 8;
    let increments_per_thread = 10_000;

    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];

    // Spawn N threads, each incrementing M times
    for _ in 0..num_threads {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..increments_per_thread {
                c.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    // Wait for all threads
    for h in handles {
        h.join().expect("Thread panicked");
    }

    let expected = (num_threads * increments_per_thread) as u64;
    let actual = counter.load(Ordering::SeqCst);

    println!(
        "Counter: {actual} (expected {expected}) using {num_threads} threads × {increments_per_thread} increments"
    );
    assert_eq!(actual, expected, "Counter should be exactly {expected}");

    // --- Bonus: Using fetch_sub ---
    counter.fetch_sub(5000, Ordering::Relaxed);
    println!("After subtracting 5000: {} (expected {})", counter.load(Ordering::Relaxed), expected - 5000);

    // --- Multiple atomic operations ---
    // Demonstrate that atomics compose safely for read-modify-write
    let flags = Arc::new(AtomicU64::new(0));
    let mut flag_handles = vec![];

    for i in 0..num_threads {
        let f = Arc::clone(&flags);
        flag_handles.push(thread::spawn(move || {
            // Each thread sets its own bit
            f.fetch_or(1 << i, Ordering::Relaxed);
        }));
    }

    for h in flag_handles {
        h.join().unwrap();
    }

    let final_flags = flags.load(Ordering::SeqCst);
    println!("Bit flags: {final_flags:#b} (all {num_threads} bits set)");
    assert_eq!(final_flags, (1 << num_threads) - 1, "All bits should be set");

    println!("All atomic counter tests passed!");
}
