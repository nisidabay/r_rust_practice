// ex04_retry.rs — BONUS
// Retry with exponential backoff pattern.
//
// When an async operation fails (network blip, rate limit, etc.),
// you typically want to retry with increasing delays between attempts.
// This is called exponential backoff.

use std::thread;
use std::time::{Duration, Instant};

/// Result of a simulated operation.
#[derive(Debug, PartialEq)]
enum OpResult {
    Success(String),
    TransientError(String),
    FatalError(String),
}

/// Simulate a flaky operation.
/// - Succeeds on attempt >= min_success_attempt
/// - Otherwise fails with a transient error until attempt >= min_success_attempt
/// - If skip >= 0 and attempt == skip, throws a fatal error
fn flaky_operation(attempt: u32, min_success_attempt: u32, skip: i32) -> OpResult {
    thread::sleep(Duration::from_millis(50)); // simulate some work

    if skip >= 0 && attempt == skip as u32 {
        return OpResult::FatalError("Invalid credentials — cannot retry".to_string());
    }

    if attempt >= min_success_attempt {
        OpResult::Success(format!("Data fetched on attempt {attempt}"))
    } else {
        OpResult::TransientError(format!("Attempt {attempt}: server timeout"))
    }
}

/// Retry an operation with exponential backoff.
/// - max_retries: max number of retry attempts
/// - base_delay_ms: initial delay between retries (doubles each time)
/// - Returns on first success or fatal error
fn retry_with_backoff(
    min_success_attempt: u32,
    max_retries: u32,
    base_delay_ms: u64,
    skip: i32,
) -> OpResult {
    let mut attempt = 1u32;

    loop {
        let result = flaky_operation(attempt, min_success_attempt, skip);
        match result {
            OpResult::Success(_) => return result,
            OpResult::FatalError(_) => return result,
            OpResult::TransientError(msg) => {
                println!("  {msg}");
                if attempt >= max_retries {
                    return OpResult::TransientError(format!("Gave up after {attempt} attempts"));
                }
                // Exponential backoff: 2^(attempt-1) * base_delay_ms
                let delay_ms = base_delay_ms * (1u64 << (attempt - 1));
                println!("  Waiting {delay_ms}ms before retry...");
                thread::sleep(Duration::from_millis(delay_ms));
                attempt += 1;
            }
        }
    }
}

fn main() {
    println!("=== ex04 (BONUS): Retry with exponential backoff ===\n");

    // Case 1: Succeeds on attempt 3
    println!("--- Case 1: Transient failures, succeeds on retry 3 ---");
    let start = Instant::now();
    let result = retry_with_backoff(3, 5, 100, -1);
    println!("  [{:?}] Result: {result:?}", start.elapsed());

    // Case 2: Fatal error — no retry
    println!("\n--- Case 2: Fatal error on attempt 2 — must not retry ---");
    let start = Instant::now();
    let result = retry_with_backoff(1, 5, 100, 2);
    println!("  [{:?}] Result: {result:?} (expected: FatalError)", start.elapsed());

    // Case 3: Exhausted retries
    println!("\n--- Case 3: All retries exhausted ---");
    let start = Instant::now();
    let result = retry_with_backoff(10, 3, 50, -1);
    println!("  [{:?}] Result: {result:?} (expected: Gave up)", start.elapsed());

    println!("\n✅ Exponential backoff pattern: retries with doubling delay, stops on fatal errors");
}
