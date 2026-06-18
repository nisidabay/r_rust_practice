// 03_futures_intro.rs
// What is a Future? The poll-based model.
//
// A Future in Rust is a state machine. Each time you poll it,
// it either returns Ready(value) or Pending. The executor
// calls poll repeatedly (when woken up) until Ready.

use std::thread;
use std::time::{Duration, Instant};

/// A simple stateful future that completes after a delay.
/// This shows the core of how Futures work internally.
struct DelayedValue {
    start: Instant,
    delay: Duration,
    started: bool,
}

impl DelayedValue {
    fn new(delay: Duration) -> Self {
        Self {
            start: Instant::now(),
            delay,
            started: false,
        }
    }

    /// Poll this future. Returns Some(value) when ready, None if still pending.
    /// This is the essence of the Future::poll() method.
    fn poll(&mut self) -> Option<&str> {
        if self.started {
            // Already completed — would be an error in real futures
            panic!("Polled a completed future!");
        }
        if self.start.elapsed() >= self.delay {
            self.started = true;
            Some("ready!")
        } else {
            None // Still pending — try again later
        }
    }
}

fn main() {
    println!("=== 03_futures_intro: The poll-based Future model ===");

    // Create a future that will be ready in 1.5 seconds
    let mut future = DelayedValue::new(Duration::from_secs_f32(1.5));

    // The executor polls the future in a loop (simplified).
    // In a real system, the waker tells the executor when to poll again.
    let mut attempts = 0;
    let result = loop {
        attempts += 1;
        match future.poll() {
            Some(val) => break val,
            None => {
                // Not ready yet — yield so other work can happen
                thread::sleep(Duration::from_millis(200));
            }
        }
    };

    println!("Future returned: {result} after {attempts} poll attempts");

    // KEY CONCEPT:
    // Between polls, the thread could do OTHER work — like polling
    // other futures. This is how a single thread handles thousands
    // of concurrent tasks. Each future is a small state machine:
    //
    //   enum FetchFuture {
    //       Start,
    //       WaitingForSocket { fd: i32, buffer: Vec<u8> },
    //       Done(String),
    //   }
    //
    // Each poll advances the state machine. No thread is parked.

    println!("\n✅ Future poll model: state machine driven by repeated polling");
}
