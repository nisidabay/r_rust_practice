// 06_unsafe_trait.rs — Implementing unsafe traits (Send, Sync manually)
//
// What do you do when you need to break the rules?
//
// An unsafe trait is a trait whose implementation requires extra invariants
// that the compiler cannot verify. `Send` and `Sync` are the classic examples:
//
//   - `Send`: a type can be safely transferred between threads
//   - `Sync`: a type can be safely shared between threads (shared reference)
//
// These are automatically derived for most types, but if you have a raw pointer
// or interior mutability, you may need to mark them manually.

use std::cell::UnsafeCell;
use std::marker::Sync;
use std::thread;

// --- A simple thread-safe wrapper built on UnsafeCell ---
// Normally UnsafeCell<T> is !Sync (not safe to share).
// But if we add a Mutex-like lock, it WOULD be Sync.
// Here we do it manually to demonstrate the pattern.

/// A fake "lock" that wraps a value. We'll manually mark it Sync
/// because we guarantee (pretend) that access is serialized.
struct MyLock<T> {
    inner: UnsafeCell<T>,
}

// SAFETY: We promise that our interface ensures exclusive access.
// In a real implementation you'd use a real Mutex or spinlock.
unsafe impl<T: Send> Sync for MyLock<T> {}

impl<T> MyLock<T> {
    fn new(val: T) -> Self {
        MyLock {
            inner: UnsafeCell::new(val),
        }
    }

    /// Pretend this acquires a lock. In reality, this is unsynchronized!
    /// Real code would use atomic flags or OS primitives.
    fn get(&self) -> *mut T {
        self.inner.get()
    }
}

// --- Defining our own unsafe trait ---
/// A trait for types that can be "safely zeroed" — the memory can be set to
/// all-zero bytes and remain valid. This is NOT true for most types
/// (e.g., Box, Vec, String have non-null pointers).
unsafe trait Zeroable: Sized {
    fn zeroed() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// u32 is safe to zero (0u32 is valid)
unsafe impl Zeroable for u32 {}
// f32 is safe to zero (0.0f32 is valid)
unsafe impl Zeroable for f32 {}

fn main() {
    println!("=== Unsafe Traits ===");

    // --- MyLock usage ---
    let lock = MyLock::new(42);
    println!("Created MyLock<i32> — manually marked Sync");

    // Since MyLock is Sync, we can share it across threads via & reference
    // (even though our lock doesn't actually synchronize — this is a demo)
    let shared: &MyLock<i32> = &lock;

    // In real code, we'd need a proper lock. This just shows the pattern:
    unsafe {
        println!(
            "Value via raw pointer: {}",
            *shared.get()
        );
    }

    // --- Zeroable trait ---
    let zero: u32 = unsafe { <u32 as Zeroable>::zeroed() };
    println!("Zeroed u32: {}", zero);
    let zero_f: f32 = unsafe { <f32 as Zeroable>::zeroed() };
    println!("Zeroed f32: {}", zero_f);

    println!();
    println!("Key points:");
    println!("  - `unsafe trait`: the implementor must uphold invariants");
    println!("  - Send/Sync are auto-derived; manually implementing them");
    println!("    bypasses the compiler's safety checks");
    println!("  - Always document SAFETY conditions for unsafe trait impls");
    println!("  - Most custom unsafe traits are for low-level or FFI code");
}
