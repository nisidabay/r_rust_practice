// 05_mut_static.rs — Mutable static variables, unsafe access
//
// What do you do when you need to break the rules?
//
// Mutable static variables are globally accessible from any thread.
// Reading or writing a mutable static is inherently unsafe because
// it can cause data races. Access requires an `unsafe` block.

/// Global mutable counter — can be read/written from anywhere.
/// Accessing this is unsafe because there's no synchronization.
static mut HITS: u64 = 0;

/// A safe wrapper providing controlled access to the mutable static.
fn increment_hits() {
    unsafe {
        HITS += 1;
    }
}

fn read_hits() -> u64 {
    unsafe { HITS }
}

// --- Safer alternative: use atomics ---
use std::sync::atomic::{AtomicU64, Ordering};

static SAFE_HITS: AtomicU64 = AtomicU64::new(0);

fn main() {
    println!("=== Mutable Statics ===");

    // Direct access requires unsafe
    unsafe {
        HITS = 42;
        println!("Direct unsafe access: HITS = {}", HITS);
    }

    // Via safe wrapper
    increment_hits();
    println!("After increment: HITS = {}", read_hits());

    // --- Atomic alternative (no unsafe needed!) ---
    println!();
    println!("--- Atomic alternative ---");
    SAFE_HITS.store(100, Ordering::SeqCst);
    SAFE_HITS.fetch_add(1, Ordering::SeqCst);
    println!(
        "SAFE_HITS = {} (no unsafe required with atomics)",
        SAFE_HITS.load(Ordering::SeqCst)
    );

    println!();
    println!("Key points:");
    println!("  - `static mut` requires unsafe for every access");
    println!("  - Prefer AtomicXxx or Mutex<T> inside a static");
    println!("  - `const` and `static` (immutable) do NOT need unsafe");
    println!("  - Mutable statics are a last resort for global state");
}
