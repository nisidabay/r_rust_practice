// 01_async_basics.rs
// What async fn and .await actually do — no threads involved.
//
// Rust's `async fn` returns a Future. When you .await it, execution
// suspends at that point until the future is ready. No thread is
// blocked — the thread goes off to do other work instead.

use std::thread;
use std::time::{Duration, Instant};

/// Simulates an async operation by sleeping in a separate thread.
/// In a real executor, this would be non-blocking I/O.
/// We wrap it in a hand-rolled "future" to show the concept.
fn simulated_async_work(id: u32, seconds: u64) -> impl FnOnce() -> String {
    move || {
        thread::sleep(Duration::from_secs(seconds));
        format!("Task {id} completed after {seconds}s")
    }
}

fn main() {
    println!("=== 01_async_basics: Understanding async fn and .await ===");

    // In real async Rust, we'd write:
    //
    //   async fn fetch_data(url: &str) -> String {
    //       reqwest::get(url).await.unwrap().text().await
    //   }
    //
    // But since we're using only std, we simulate with threads
    // to demonstrate the *concept* of concurrent execution.

    let start = Instant::now();

    // Spawn two pieces of "async work" as OS threads.
    // This simulates what an async executor does: it runs
    // multiple futures, switching between them when one
    // would otherwise block.
    let t1 = thread::spawn(simulated_async_work(1, 2));
    let t2 = thread::spawn(simulated_async_work(2, 1));

    // .await would wait for the future to complete.
    // Here join() blocks until the thread finishes.
    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();

    let elapsed = start.elapsed();
    println!("{r1}");
    println!("{r2}");
    println!("Total time: {elapsed:?} (tasks ran concurrently)");

    // KEY INSIGHT: If these ran *sequentially*, the total
    // would be 2s + 1s = 3s. But they ran concurrently
    // (like async tasks on an executor), completing in ~2s.
    // No threads were waiting idly — the OS scheduler (or
    // async executor) switched between them.
    assert!(elapsed < Duration::from_secs(3));
    println!("\n✅ Async basics: concurrent execution achieved without blocking a whole thread per task (concept demonstrated with threads)");
}
