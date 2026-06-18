// 02_block_on.rs
// A minimal "executor" that polls a future to completion.
//
// In production you'd use tokio::runtime::Runtime or
// futures::executor::block_on. Here we build the simplest
// possible version to show the polling loop.

use std::thread;
use std::time::{Duration, Instant};

/// A minimal Future trait for demonstration.
/// In std::future, the real Future::poll takes Pin<&mut Self>
/// and a Context (waker). This simplified version captures
/// the core idea: a future returns Pending until Ready.
#[allow(dead_code)]
enum SimFuture<T> {
    Pending,
    Ready(T),
}

/// Our miniature "block_on" executor.
/// It takes a closure (representing a future) and repeatedly
/// calls it until it returns Ready.
fn block_on_sim<F, T>(mut future: F) -> T
where
    F: FnMut() -> SimFuture<T>,
{
    loop {
        match future() {
            SimFuture::Ready(val) => return val,
            SimFuture::Pending => {
                // In a real executor, this is where we'd go
                // poll OTHER futures. Here we just yield
                // the CPU briefly to avoid a busy loop.
                thread::yield_now();
            }
        }
    }
}

fn main() {
    println!("=== 02_block_on: How an executor drives futures ===");

    let start = Instant::now();

    // Simulate an async operation: a future that takes 2s
    // In a real system, wakers notify the executor when I/O
    // is ready, so there's no busy-looping.
    let result = block_on_sim(|| {
        // Check if time has elapsed (simulating I/O readiness)
        if start.elapsed() >= Duration::from_secs(2) {
            SimFuture::Ready("Data loaded!")
        } else {
            SimFuture::Pending
        }
    });

    println!("Result: {result} in {:?}", start.elapsed());

    // The real pattern (only works with futures crate / tokio):
    //
    //   use futures::executor::block_on;
    //
    //   async fn fetch() -> String {
    //       // ... some async work ...
    //       "data".to_string()
    //   }
    //
    //   let result = block_on(fetch());
    //
    // block_on() creates a minimal runtime and polls the future
    // to completion — the same loop concept we demonstrated above.

    println!("\n✅ block_on concept: executor polls future until Ready");
}
