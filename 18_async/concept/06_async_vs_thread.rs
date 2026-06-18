// 06_async_vs_thread.rs
// Compare async with threads: overhead, switching, tradeoffs.
//
// OS threads vs async tasks:
//   - Thread:  ~1-8 MB stack per thread, kernel-managed scheduling
//   - Async:   ~few hundred bytes per task, user-space scheduling
//
// When to use each:
//   - Threads: CPU-bound work, need true parallelism, FFI that blocks
//   - Async:   I/O-bound work, thousands of concurrent connections,
//              embedded/constrained environments

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("=== 06_async_vs_thread: Comparing the two concurrency models ===");

    // --- Part 1: Thread overhead ---
    println!("\n--- Part 1: Measuring thread creation overhead ---");

    let start = Instant::now();
    let handles: Vec<_> = (0..100)
        .map(|i| {
            thread::spawn(move || {
                // Minimal work
                i * 2
            })
        })
        .collect();

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    let thread_time = start.elapsed();
    println!("100 threads created, worked, joined in {thread_time:?}");
    println!("Thread overhead: {thread_time:?} for 100 threads = {:?} per thread", thread_time / 100);

    // --- Part 2: Context switching cost ---
    println!("\n--- Part 2: Context switch cost ---");

    // Two threads ping-ponging with a shared counter
    // This is expensive because of kernel-mode context switches.
    const ITERS: u64 = 10_000;
    let counter = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let c2 = counter.clone();

    let start = Instant::now();
    let t1 = thread::spawn(move || {
        for _ in 0..ITERS {
            c2.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            thread::yield_now(); // Force context switch
        }
    });
    let t2 = thread::spawn(move || {
        for _ in 0..ITERS {
            counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            thread::yield_now();
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
    let switch_time = start.elapsed();
    println!(
        "{ITERS} forced context switches (yield) in {switch_time:?} = ~{:?} per switch",
        switch_time / ITERS as u32
    );

    // --- Part 3: Memory comparison ---
    println!("\n--- Part 3: Memory comparison (conceptual) ---");
    println!("OS Thread stack:     ~8 MB (default on Linux)");
    println!("Async task state:    ~0.5-2 KB per task (just the state machine)");
    println!(
        "Ratio:               ~4000-16000x less memory per async task"
    );
    println!(
        "With 1 GB memory:    ~128 threads    vs    ~500,000+ async tasks"
    );

    // --- Summary ---
    println!("\n--- Summary: When to use which ---");
    println!("┌────────────────────┬─────────────────────┬─────────────────────┐");
    println!("│ Criterion          │ OS Threads          │ Async Tasks         │");
    println!("├────────────────────┼─────────────────────┼─────────────────────┤");
    println!("│ Memory per task    │ ~8 MB               │ ~1 KB               │");
    println!("│ Task creation       │ ~1-10 µs           │ ~ns                 │");
    println!("│ Context switch     │ Kernel (slow)       │ User-space (fast)   │");
    println!("│ CPU-bound work     │ ✅ Good             │ ❌ Poor             │");
    println!("│ I/O-bound work     │ ❌ Wastes memory    │ ✅ Excellent        │");
    println!("│ True parallelism   │ ✅ Yes (multi-core) │ ❌ Single-thread*   │");
    println!("│ Complexity         │ Simple              │ Med (executor+type) │");
    println!("└────────────────────┴─────────────────────┴─────────────────────┘");
    println!("* Async tasks can run on multiple threads via a multi-threaded executor.");
    println!();
    println!("✅ Tradeoffs understood: threads for CPU/parallelism, async for I/O concurrency");
}
