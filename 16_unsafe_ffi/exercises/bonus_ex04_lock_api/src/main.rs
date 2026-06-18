// BONUS: ex04_lock_api — Build a tiny safe API around unsafe FFI
//
// A spinlock using atomic operations and raw pointers internally,
// but exposing a fully safe API. This demonstrates the classic pattern:
// wrap unsafe guts in a safe interface.

use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

/// A simple spinlock — wraps a value and provides exclusive access.
/// The internals use unsafe (UnsafeCell + raw pointer), but the API is safe.
pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

/// The guard returned by SpinLock::lock().
/// When dropped, it unlocks the spinlock.
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    /// Create a new spinlock. Safe — no unsafe needed by the caller.
    pub fn new(val: T) -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(val),
        }
    }

    /// Acquire the lock (spins until available).
    /// Returns a guard that provides Deref/DerefMut to the inner value.
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        // Spin until we acquire the lock
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Hint to CPU that we're spinning (x86 pause instruction)
            std::hint::spin_loop();
        }
        SpinLockGuard { lock: self }
    }

    /// Get a raw mutable pointer (unsafe — internal use)
    fn get_ptr(&self) -> *mut T {
        self.data.get()
    }
}

// SAFETY: SpinLock<T> is Send/Sync if T is Send (the lock provides mutual exclusion)
unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> Drop for SpinLock<T> {
    fn drop(&mut self) {
        // No need to unlock — all guards should be dropped by now
    }
}

impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.get_ptr() }
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.get_ptr() }
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

// --- Safe multi-threaded demo ---

fn main() {
    println!("=== BONUS: SpinLock — safe API around unsafe internals ===");

    let lock = SpinLock::new(0);
    println!("Created SpinLock<i32> with initial value 0");

    // Test basic usage
    {
        let mut guard = lock.lock();
        *guard += 10;
        println!("After guard += 10: {}", *guard);
    } // guard dropped, lock released

    {
        let guard = lock.lock();
        println!("Re-acquired: {}", *guard);
    }

    // Test with heap-allocated data
    let vec_lock = SpinLock::new(vec![1, 2, 3]);
    {
        let mut guard = vec_lock.lock();
        guard.push(4);
        guard.push(5);
        println!("Vec inside lock: {:?}", *guard);
    }

    // Multi-threaded test using scoped threads
    let counter = SpinLock::new(0u64);
    std::thread::scope(|s| {
        for _ in 0..8 {
            s.spawn(|| {
                for _ in 0..1000 {
                    let mut guard = counter.lock();
                    *guard += 1;
                }
            });
        }
    });

    let final_val = *counter.lock();
    println!(
        "8 threads incrementing 1000x each: {} (expected 8000)",
        final_val
    );
    assert_eq!(final_val, 8000);

    println!();
    println!("All assertions passed! ✓");
    println!();
    println!("Architecture recap:");
    println!("  1. UnsafeCell<T> provides interior mutability (safe to get &mut T from &Self)");
    println!("  2. AtomicBool provides the lock state (no mutex needed for simple spinlock)");
    println!("  3. SpinLockGuard uses Deref/DerefMut to expose &T/&mut T safely");
    println!("  4. Guard Drop releases the lock — RAII ensures safety");
    println!("  5. Unsafe Send/Sync impls because we guarantee mutual exclusion");
}
