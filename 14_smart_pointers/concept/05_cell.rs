// 05_cell.rs — Cell<T>, Copy types interior mutability
//
// Cell<T> provides interior mutability for Copy types (like integers,
// bools, chars, and types that implement Copy). Unlike RefCell, Cell
// doesn't do runtime borrow checking — it's always safe because it
// never hands out references to internal data. Instead, it returns
// copies: get() copies the value out, set() replaces it entirely.
//
// Cell is lighter than RefCell (no borrow tracking overhead) but only
// works with Copy types. For non-Copy types, use RefCell.

use std::cell::Cell;

fn main() {
    // Basic Cell usage — integer counter
    let counter = Cell::new(0);

    // set() replaces the entire value
    counter.set(10);
    assert_eq!(counter.get(), 10);

    // update() applies a closure to the inner value (nightly feature)
    // Use get/set manually for stable Rust:
    counter.set(counter.get() + 5);
    assert_eq!(counter.get(), 15);

    // The struct below shows how Cell enables &self mutation methods.

    // Practical: tracking stats counters
    let stats = Stats::new();
    stats.record_hit();
    stats.record_hit();
    stats.record_miss();
    stats.record_hit();
    println!("Hits: {}, Misses: {}", stats.hits(), stats.misses());

    // Practical: memoization with Cell for simple computed values
    // (for complex types you'd use RefCell, but for Copy types Cell is perfect)
    let memo = MemoizedValue::new(42);
    println!("Computed (first): {}", memo.get_or_compute());
    println!("Computed (cached): {}", memo.get_or_compute());

    // Practical: flags / toggles
    let flag = Cell::new(false);
    toggle(&flag);
    println!("Flag after toggle: {}", flag.get());
    toggle(&flag);
    println!("Flag after second toggle: {}", flag.get());
}

/// Stats tracker: uses Cell for efficient interior mutation.
/// All methods take &self, not &mut self.
struct Stats {
    hits: Cell<u64>,
    misses: Cell<u64>,
}

impl Stats {
    fn new() -> Self {
        Stats {
            hits: Cell::new(0),
            misses: Cell::new(0),
        }
    }

    fn record_hit(&self) {
        self.hits.set(self.hits.get() + 1);
    }

    fn record_miss(&self) {
        self.misses.set(self.misses.get() + 1);
    }

    fn hits(&self) -> u64 {
        self.hits.get()
    }

    fn misses(&self) -> u64 {
        self.misses.get()
    }
}

/// Memoized computation: stores a cached result using Cell.
struct MemoizedValue {
    input: i32,
    cached: Cell<Option<i32>>,
}

impl MemoizedValue {
    fn new(input: i32) -> Self {
        MemoizedValue {
            input,
            cached: Cell::new(None),
        }
    }

    /// Returns the computed value, cached after first call.
    fn get_or_compute(&self) -> i32 {
        match self.cached.get() {
            Some(val) => {
                println!("  (returning cached)");
                val
            }
            None => {
                let result = self.input * self.input; // expensive computation
                self.cached.set(Some(result));
                result
            }
        }
    }
}

/// Toggle a boolean flag — Cell allows mutation through & reference.
fn toggle(flag: &Cell<bool>) {
    flag.set(!flag.get());
}

// Cell vs RefCell comparison:
//
//   Cell<T>                    RefCell<T>
//   ───────                    ─────────
//   T: Copy                    T: any Sized type
//   No borrow checking         Runtime borrow checking
//   get() returns copy         borrow() returns Ref<T>
//   set() replaces value       borrow_mut() returns RefMut<T>
//   No panic risk              Panics on rule violation
//   Lightweight                Slightly more overhead
//   Best for counters, flags   Best for complex objects
