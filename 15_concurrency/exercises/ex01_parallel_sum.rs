// ex01_parallel_sum.rs — Sum a large vector using multiple threads (chunks)
//
// Task: Given a large vector of u64 values, split it into chunks and sum
// each chunk in a separate thread. Collect results and compute the total.
//
// The one question: "How do you run code in parallel — safely?"
// Answer: Split data into disjoint chunks, process each in its own thread,
// then merge results. No shared state needed — just message passing or
// collecting JoinHandle results.

use std::thread;

/// Sum a slice using multiple threads.
/// Returns the total sum and the number of threads used.
fn parallel_sum(data: &[u64], num_threads: usize) -> (u64, usize) {
    if data.is_empty() {
        return (0, 1);
    }

    // Determine chunk size
    let chunk_size = (data.len() + num_threads - 1) / num_threads; // ceiling division
    let actual_threads = (data.len() + chunk_size - 1) / chunk_size;

    let mut handles = vec![];

    for chunk_start in (0..data.len()).step_by(chunk_size) {
        let chunk_end = std::cmp::min(chunk_start + chunk_size, data.len());
        let chunk = data[chunk_start..chunk_end].to_vec(); // Copy for the thread

        handles.push(thread::spawn(move || {
            chunk.iter().sum::<u64>()
        }));
    }

    // Collect results
    let total: u64 = handles
        .into_iter()
        .map(|h| h.join().expect("Thread panicked"))
        .sum();

    (total, actual_threads)
}

fn main() {
    // --- Test with a small vector that we can verify ---
    let data: Vec<u64> = (1..=100).collect(); // sum = 5050

    let (total, threads) = parallel_sum(&data, 4);
    println!("Sum 1..100 = {total} (expected 5050) using {threads} threads");
    assert_eq!(total, 5050, "Sum should be 5050");

    // --- Test with a larger vector ---
    let big_data: Vec<u64> = (1..=1_000_000).collect();
    let expected: u64 = (1_000_000u64 * 1_000_001) / 2;

    let (big_sum, big_threads) = parallel_sum(&big_data, 8);
    println!("Sum 1..1,000,000 = {big_sum} (expected {expected}) using {big_threads} threads");
    assert_eq!(big_sum, expected, "Sum should be {expected}");

    // --- Empty vector ---
    let (empty_sum, _) = parallel_sum(&[], 4);
    assert_eq!(empty_sum, 0, "Empty sum should be 0");

    // --- Single element ---
    let (single_sum, _) = parallel_sum(&[42], 4);
    assert_eq!(single_sum, 42, "Single element sum should be 42");

    // --- Fewer elements than threads ---
    let (small_sum, _) = parallel_sum(&[1, 2, 3], 10);
    assert_eq!(small_sum, 6, "Small vector sum should be 6");

    println!("All tests passed!");
}
