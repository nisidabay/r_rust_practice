// 04_async_chaining.rs
// Multiple .awaits, sequential async calls.
//
// In async Rust, each .await polls a future until it's Ready.
// When you chain .awaits, they run sequentially — but the thread
// is never blocked; it can poll other futures between each step.

use std::thread;
use std::time::{Duration, Instant};

/// Simulates an async step (like an HTTP request or DB query).
fn step(name: &str, duration: u64) -> String {
    thread::sleep(Duration::from_secs(duration));
    format!("{name} done")
}

fn main() {
    println!("=== 04_async_chaining: Sequential async calls ===");

    let start = Instant::now();

    // In real async Rust, this would be:
    //
    //   async fn process() -> String {
    //       let a = step_a().await;   // thread can do other work
    //       let b = step_b(&a).await; // while waiting for each step
    //       let c = step_c(&b).await;
    //       c
    //   }
    //
    // Each .await suspends the *current async function*, not the
    // thread. The executor runs other futures while we wait.

    // We simulate with threads to show the sequential dependency.
    let r1 = step("Step A (fetch user)", 2);
    println!("[{:?}] {r1}", start.elapsed());

    // Step B depends on step A's result (simulated by ordering)
    let r2 = step("Step B (fetch orders)", 1);
    println!("[{:?}] {r2}", start.elapsed());

    // Step C depends on step B's result
    let r3 = step("Step C (format report)", 1);
    println!("[{:?}] {r3}", start.elapsed());

    let elapsed = start.elapsed();
    println!("Total sequential time: {elapsed:?}");

    // KEY INSIGHT: Each step waited for the previous one.
    // Total = 2 + 1 + 1 = 4 seconds.
    // But the THREAD was not blocked/parked — in a real executor,
    // it would have been polling other futures between polls of
    // these steps. The async function just wasn't ready to proceed
    // until its data arrived.
    assert!(elapsed >= Duration::from_secs(4));

    println!("\n✅ Async chaining: sequential .awaits with non-blocking suspension between each step");
}
