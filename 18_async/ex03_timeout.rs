// ex03_timeout.rs
// Race between work and timeout — the select! pattern.
//
// In real async Rust, tokio::select! or futures::future::select
// runs two futures and uses the first one that completes.
// Here we demonstrate with threads and a channel.

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// Run some work and send its result through a channel.
/// Simulates an async task that might take too long.
fn do_work(result_tx: mpsc::Sender<&'static str>, delay_ms: u64) {
    thread::sleep(Duration::from_millis(delay_ms));
    let _ = result_tx.send("work completed");
}

/// A timeout that sends a timeout signal after a delay.
fn timeout(tx: mpsc::Sender<&'static str>, delay_ms: u64) {
    thread::sleep(Duration::from_millis(delay_ms));
    let _ = tx.send("TIMEOUT");
}

/// Race two closures and return the first result.
/// This is the concept behind futures::future::select / tokio::select!.
fn race<A, B>(work: A, timeout: B) -> &'static str
where
    A: FnOnce() -> &'static str + Send + 'static,
    B: FnOnce() -> &'static str + Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let result = work();
        let _ = tx.send(result);
    });
    thread::spawn(move || {
        let result = timeout();
        let _ = tx2.send(result);
    });

    // Block until first result arrives — this is the "select" moment
    rx.recv().unwrap()
}

fn main() {
    println!("=== ex03: Timeout pattern — race work vs timeout ===\n");

    // Case 1: Work finishes before timeout
    println!("--- Case 1: Work finishes in time ---");
    let start = Instant::now();
    let result = race(
        || {
            do_work(mpsc::channel().0, 500); // just for delay effect
            "data loaded"
        },
        || {
            thread::sleep(Duration::from_millis(1000));
            "TIMEOUT"
        },
    );
    println!("  [{:?}] Result: {result} (expected: work wins)", start.elapsed());

    // Case 2: Work is too slow, timeout kicks in
    println!("\n--- Case 2: Work is too slow, timeout wins ---");
    let start = Instant::now();
    let result = race(
        || {
            thread::sleep(Duration::from_millis(1500));
            "data loaded"
        },
        || {
            thread::sleep(Duration::from_millis(500));
            "TIMEOUT"
        },
    );
    println!("  [{:?}] Result: {result} (expected: timeout wins)", start.elapsed());

    println!("\n✅ Timeout pattern: the race completes in min(work, timeout) time");
}
