// 05_join_concurrent.rs
// Concurrent execution concept — the join pattern.
//
// join! runs multiple futures concurrently and waits for ALL to
// complete. This is the foundation of fan-out concurrency in async.

use std::thread;
use std::time::{Duration, Instant};

/// Simulated async work: sleeps and returns a result.
fn simulated_fetch(id: u32, seconds: u64) -> String {
    thread::sleep(Duration::from_secs(seconds));
    format!("Data source {}: {} records", id, id * 1000)
}

/// Run two tasks concurrently and wait for both.
/// This is what `futures::future::join(t1, t2).await` does.
fn join<A: Send + 'static, B: Send + 'static>(
    f1: impl FnOnce() -> A + Send + 'static,
    f2: impl FnOnce() -> B + Send + 'static,
) -> (A, B) {
    let t1 = thread::spawn(f1);
    let t2 = thread::spawn(f2);
    (t1.join().unwrap(), t2.join().unwrap())
}

fn main() {
    println!("=== 05_join_concurrent: Running futures concurrently ===");

    let start = Instant::now();

    // In real async Rust:
    //
    //   use futures::future::join;
    //
    //   async fn fetch_all() -> (String, String) {
    //       join!(fetch_users(), fetch_orders()).await
    //   }
    //
    // join! polls BOTH futures on every executor cycle.
    // When one is Pending, the other gets polled.
    // Both complete in max(t1, t2) time, not t1 + t2.

    let (r1, r2) = join(
        || simulated_fetch(1, 3), // takes 3s
        || simulated_fetch(2, 2), // takes 2s
    );

    let elapsed = start.elapsed();

    println!("{r1}");
    println!("{r2}");
    println!("Total time: {elapsed:?}");

    // If sequential: 3 + 2 = 5s. If concurrent: max(3, 2) = ~3s.
    assert!(elapsed < Duration::from_secs(4));
    println!("Concurrent execution: completed in max(t1, t2) time, not t1 + t2");

    // For multiple futures, use join_all (a single future that
    // polls all children):
    //
    //   use futures::future::join_all;
    //
    //   let results: Vec<String> = join_all(tasks).await;

    println!("\n✅ Join pattern: concurrent execution of independent futures");
}
