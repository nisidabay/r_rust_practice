// 06_benchmarking.rs — Simple timing, iter().count() patterns
//
// Rust has unstable #[bench] on nightly. Here we show manual timing patterns
// that work on stable Rust with stdlib only.
//
// Run: rustc --edition 2021 --test 06_benchmarking.rs && ./06_benchmarking

use std::time::{Duration, Instant};

// --- Simple timing helper ---

/// Measures how long a closure takes to run, returns Duration.
fn time_it<F>(f: F) -> Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    start.elapsed()
}

/// Runs a closure `n` times and returns the average duration.
fn bench_n<F>(n: u32, mut f: F) -> Duration
where
    F: FnMut(),
{
    let start = Instant::now();
    for _ in 0..n {
        f();
    }
    let total = start.elapsed();
    Duration::from_nanos(total.as_nanos() as u64 / n as u64)
}

// --- Functions to benchmark ---

/// Sums numbers 1..n using a loop.
fn sum_loop(n: u64) -> u64 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}

/// Sums numbers 1..n using the formula n*(n+1)/2.
fn sum_formula(n: u64) -> u64 {
    n * (n + 1) / 2
}

/// Collects numbers into a Vec and counts evens.
fn count_evens_collect(n: u64) -> usize {
    (1..=n).collect::<Vec<_>>().iter().filter(|&&x| x % 2 == 0).count()
}

/// Counts evens using iterator only (no intermediate Vec).
fn count_evens_iter(n: u64) -> usize {
    (1..=n).filter(|x| x % 2 == 0).count()
}

// --- Tests that also benchmark ---

#[test]
fn test_and_bench() {
    // First verify correctness
    assert_eq!(sum_loop(100), 5050);
    assert_eq!(sum_formula(100), 5050);
    assert_eq!(count_evens_collect(100), 50);
    assert_eq!(count_evens_iter(100), 50);

    // Then benchmark (informal — print timing)
    let n = 1_000_000u64;

    let t1 = bench_n(10, || {
        let _ = sum_loop(n);
    });
    let t2 = bench_n(10, || {
        let _ = sum_formula(n);
    });

    println!("sum_loop({n})  avg: {:?}", t1);
    println!("sum_formula({n}) avg: {:?}", t2);

    let t3 = bench_n(10, || {
        let _ = count_evens_collect(n);
    });
    let t4 = bench_n(10, || {
        let _ = count_evens_iter(n);
    });

    println!("count_evens_collect({n}) avg: {:?}", t3);
    println!("count_evens_iter({n})    avg: {:?}", t4);

    // The formula should be way faster than the loop
    assert!(t2 < t1 || t2.as_nanos() < t1.as_nanos() * 10,
        "formula should be much faster than loop");
}

#[test]
fn test_single_shot_timing() {
    // A quick one-shot timing for simple operations
    let d = time_it(|| {
        let _: Vec<_> = (0..10_000).collect();
    });
    println!("collect 10k items: {:?}", d);
    // Should complete in reasonable time
    assert!(d < Duration::from_secs(1),
        "collecting 10k items should take less than 1 second");
}
