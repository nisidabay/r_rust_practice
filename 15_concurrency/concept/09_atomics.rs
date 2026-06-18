// 09_atomics.rs — AtomicBool, AtomicU32, Ordering basic intro
//
// The one question: "How do you run code in parallel — safely?"
// Answer: Use atomics for lock-free, thread-safe operations on primitives.
//
// Atomics provide operations that complete in a single hardware instruction,
// so they're safe to use from multiple threads without a Mutex.
// The Ordering parameter controls memory ordering guarantees (Relaxed, Acquire,
// Release, AcqRel, SeqCst).

use std::sync::atomic::{
    AtomicBool, AtomicU32, AtomicU64,
    Ordering::{Acquire, Relaxed, Release, SeqCst},
};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    // --- AtomicBool ---
    println!("=== AtomicBool ===");
    atomic_bool_example();

    // --- AtomicU32 counter ---
    println!("\n=== AtomicU32 counter ===");
    atomic_counter_example();

    // --- Fetch-and-modify operations ---
    println!("\n=== Fetch-and-modify ===");
    fetch_modify_example();

    // --- Compare-and-swap (CAS) ---
    println!("\n=== Compare-and-swap ===");
    cas_example();

    // --- Memory ordering demonstration ---
    println!("\n=== Memory ordering ===");
    ordering_example();

    println!("\nAll atomics examples done!");
}

fn atomic_bool_example() {
    let flag = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];

    // Spawn 5 threads, each tries to set the flag to true
    for id in 0..5 {
        let flag = Arc::clone(&flag);
        handles.push(thread::spawn(move || {
            // swap sets the value and returns the previous one
            let prev = flag.swap(true, SeqCst);
            if !prev {
                println!("Thread {id} was first to set the flag!");
            } else {
                println!("Thread {id} saw flag already set");
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // Reset and use store/load
    flag.store(false, Relaxed);
    assert!(!flag.load(Relaxed));
    flag.store(true, Relaxed);
    assert!(flag.load(Relaxed));
    println!("AtomicBool final value: {}", flag.load(SeqCst));
}

fn atomic_counter_example() {
    let counter = Arc::new(AtomicU32::new(0));

    let mut handles = vec![];
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                c.fetch_add(1, Relaxed);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!(
        "Counter after 10 threads × 1000 increments: {} (expected 10000)",
        counter.load(SeqCst)
    );

    // fetch_sub
    counter.fetch_sub(500, Relaxed);
    println!("After subtracting 500: {}", counter.load(SeqCst));
}

fn fetch_modify_example() {
    let val = Arc::new(AtomicU64::new(100));

    // fetch_add: adds and returns the OLD value
    let old = val.fetch_add(25, Relaxed);
    println!("fetch_add(25): old={old}, new={}", val.load(Relaxed));

    // fetch_sub: subtracts and returns the OLD value
    let old = val.fetch_sub(10, Relaxed);
    println!("fetch_sub(10): old={old}, new={}", val.load(Relaxed));

    // fetch_and: bitwise AND
    let old = val.fetch_and(0xFF, Relaxed);
    println!("fetch_and(0xFF): old={old}, new={}", val.load(Relaxed));

    // fetch_or: bitwise OR
    let old = val.fetch_or(0xF00, Relaxed);
    println!("fetch_or(0xF00): old={old}, new={}", val.load(Relaxed));

    // fetch_xor: bitwise XOR
    let old = val.fetch_xor(0x0FF, Relaxed);
    println!("fetch_xor(0x0FF): old={old:#x}, new={:#x}", val.load(Relaxed));
}

fn cas_example() {
    // compare_exchange: atomically compare and conditionally swap
    // Returns Result<old_value, old_value> — Ok if the swap happened, Err if not
    let atomic = Arc::new(AtomicU32::new(42));

    // Try to swap 42 → 100 (succeeds because current value is 42)
    let result = atomic.compare_exchange(42, 100, SeqCst, SeqCst);
    match result {
        Ok(old) => println!("CAS success: swapped {old} for 100"),
        Err(actual) => println!("CAS failed: current value is {actual}, not 42"),
    }

    // Try to swap 42 → 200 (fails because current value is now 100)
    let result = atomic.compare_exchange(42, 200, SeqCst, SeqCst);
    match result {
        Ok(old) => println!("CAS success: swapped {old}"),
        Err(actual) => println!("CAS failed: current value is {actual}, not 42"),
    }

    // compare_exchange_weak: spurious failures allowed (more efficient on some archs)
    // Use in a loop for lock-free algorithms
    let counter = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            // Each thread increments using CAS in a loop
            loop {
                let current = c.load(Relaxed);
                if current >= 100 {
                    break;
                }
                // Try to swap current → current+1
                if c.compare_exchange_weak(current, current + 1, Relaxed, Relaxed)
                    .is_ok()
                {
                    break; // Successfully incremented
                }
                // If CAS failed (another thread changed the value), retry
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("CAS counter reached: {}", counter.load(SeqCst));
}

fn ordering_example() {
    // --- Memory ordering notes ---
    // Relaxed: No ordering guarantees — just atomicity. Fastest.
    // Acquire: Prevents subsequent reads from moving before this operation.
    // Release: Prevents prior writes from moving after this operation.
    // AcqRel: Acquire + Release (for read-modify-write ops).
    // SeqCst: Sequential consistency — strongest guarantee, slowest.

    // Simple demonstration: spinlock using AtomicBool
    let lock = Arc::new(AtomicBool::new(false));
    let data = Arc::new(AtomicU32::new(0));

    let lock2 = Arc::clone(&lock);
    let data2 = Arc::clone(&data);

    let writer = thread::spawn(move || {
        // Simulate some computation
        thread::sleep(Duration::from_millis(5));
        data2.store(42, Relaxed);

        // Release barrier: all prior writes (data2=42) are visible
        // to any thread that acquires the lock
        lock2.store(true, Release);
    });

    // Spin waiting for the lock
    while !lock.load(Acquire) {
        // Acquire barrier ensures we see data2.store(42) before continuing
        thread::yield_now();
    }

    println!("Data value after release/acquire: {}", data.load(Relaxed));

    writer.join().unwrap();

    // Summary: for simple counters, Relaxed is usually fine.
    // For flags/signaling, use Acquire/Release pairs.
    // When in doubt, use SeqCst — it's correct, just slightly slower.
}
