// ex04_parallel_map.rs — Apply a function to each element in parallel, collect results
//
// BONUS exercise.
//
// Task: Given a Vec<T> and a function Fn(T) -> R (that is Send + Clone),
// process each element in parallel across N threads, collecting results in order.
//
// The one question: "How do you run code in parallel — safely?"
// Answer: Chunk the input, process each chunk in a thread, and merge results
// in the original order. Each thread returns its chunk's results via JoinHandle.

use std::thread;

/// Apply `f` to each element of `data` in parallel using `num_threads` threads.
/// Returns results in the original order.
fn parallel_map<T, R, F>(data: Vec<T>, num_threads: usize, f: F) -> Vec<R>
where
    T: Send + Clone + 'static,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + Clone + 'static,
{
    if data.is_empty() {
        return vec![];
    }

    let n = data.len();
    let chunk_size = (n + num_threads - 1) / num_threads; // ceiling division
    let actual_threads = (n + chunk_size - 1) / chunk_size;

    let mut handles = vec![];

    for chunk_start in (0..n).step_by(chunk_size) {
        let chunk_end = std::cmp::min(chunk_start + chunk_size, n);
        let chunk: Vec<T> = data[chunk_start..chunk_end].to_vec();
        let f = f.clone();

        handles.push(thread::spawn(move || {
            // Process each element in this chunk and return the results
            chunk.into_iter().map(|x| f(x)).collect::<Vec<R>>()
        }));
    }

    // Collect in order
    let mut results = Vec::with_capacity(n);
    for (i, h) in handles.into_iter().enumerate() {
        let chunk_results = h.join().expect("Thread panicked");
        println!("  Chunk {i}: processed {} elements", chunk_results.len());
        results.extend(chunk_results);
    }

    results
}

fn main() {
    // --- Test 1: Double each number ---
    println!("Test 1: Double each number");
    let numbers: Vec<i32> = (1..=20).collect();
    let doubled = parallel_map(numbers, 4, |x| x * 2);
    let expected: Vec<i32> = (1..=20).map(|x| x * 2).collect();
    println!("  Results: {doubled:?}");
    assert_eq!(doubled, expected);
    println!("  PASS\n");

    // --- Test 2: Square and convert to string ---
    println!("Test 2: Square and convert to string");
    let nums: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let squared: Vec<String> = parallel_map(nums, 3, |x| format!("{}^2={}", x, x * x));
    println!("  Results: {squared:?}");
    assert_eq!(squared[0], "1^2=1");
    assert_eq!(squared[7], "8^2=64");
    println!("  PASS\n");

    // --- Test 3: Empty input ---
    println!("Test 3: Empty input");
    let empty: Vec<i32> = vec![];
    let result: Vec<i32> = parallel_map(empty, 4, |x| x + 1);
    assert!(result.is_empty());
    println!("  PASS\n");

    // --- Test 4: Single element ---
    println!("Test 4: Single element");
    let single = parallel_map(vec![42], 4, |x| x * 10);
    assert_eq!(single, vec![420]);
    println!("  PASS\n");

    // --- Test 5: Heavy computation (simulated) ---
    println!("Test 5: Heavy-ish computation (is_prime)");
    let numbers: Vec<u32> = (1..=30).collect();

    fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }
        let limit = (n as f64).sqrt() as u32;
        for i in 2..=limit {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    let results = parallel_map(numbers, 6, is_prime);
    let expected: Vec<bool> = (1..=30).map(is_prime).collect();
    println!("  Primes in 1..30: {:?}", (1..=30).zip(&results).filter(|(_, &p)| p).map(|(n, _)| n).collect::<Vec<_>>());
    assert_eq!(results, expected);
    println!("  PASS\n");

    println!("All parallel_map tests passed!");
}
