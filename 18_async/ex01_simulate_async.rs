// ex01_simulate_async.rs
// Simulate async with thread::sleep, demonstrate non-blocking concept.
//
// Run: rustc --edition 2021 ex01_simulate_async.rs && ./ex01_simulate_async

use std::thread;
use std::time::{Duration, Instant};

/// Simulates an async I/O operation (like reading a file or HTTP request).
/// The actual work runs on a separate thread, and we "await" it via join.
/// In real async, no OS thread is consumed — just a state machine.
fn fetch_data(id: u32, delay_s: u64) -> thread::JoinHandle<String> {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(delay_s));
        format!("Data from source {id} (took {delay_s}s)")
    })
}

fn main() {
    println!("=== ex01: Simulating async I/O ===");
    println!("Spawning 5 simulated async operations...\n");

    let start = Instant::now();

    // "Launch" all async tasks — they start immediately
    let tasks = vec![
        fetch_data(1, 3),
        fetch_data(2, 1),
        fetch_data(3, 2),
        fetch_data(4, 4),
        fetch_data(5, 1),
    ];

    // "Await" all of them (join them back)
    for (i, task) in tasks.into_iter().enumerate() {
        let result = task.join().unwrap();
        println!("[{:?}] Task {} completed: {result}", start.elapsed(), i + 1);
    }

    let total = start.elapsed();
    println!("\nTotal time: {total:?}");

    // KEY DEMONSTRATION: If we ran these sequentially, it would take
    // 3 + 1 + 2 + 4 + 1 = 11 seconds. But because they ran concurrently
    // (like async tasks), it took max(3, 1, 2, 4, 1) = ~4 seconds.
    assert!(
        total < Duration::from_secs(5),
        "Concurrent execution should complete in ~max(delay) time"
    );
    println!("✅ Concurrent execution: {total:?} vs 11s sequential");
}
