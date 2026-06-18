// 02_generic_struct.rs
// Generic structs — defining data structures that work with any type.
//
// A single struct definition with type parameters can be instantiated with
// different concrete types. The `impl<T>` block makes methods available for
// all T, while `impl` blocks with specific bounds apply only to certain types.
//
// Run: rustc --edition 2021 02_generic_struct.rs && ./02_generic_struct

use std::fmt::Display;

/// A simple generic Point<T> that holds x and y coordinates of the same type.
#[derive(Debug, Clone, Copy)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    /// Create a new Point. Works for any T.
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// Methods only available when T implements Display — demonstrates trait bounds
// on impl blocks for generic structs.
impl<T: Display> Point<T> {
    /// Print the point as (x, y).
    fn show(&self) {
        println!("Point({}, {})", self.x, self.y);
    }
}

/// A Pair<T, U> holds two values that may have different types.
#[derive(Debug, Clone, Copy)]
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    /// Destructure into a tuple.
    fn into_tuple(self) -> (T, U) {
        (self.first, self.second)
    }
}

/// A generic wrapper that only stores a value; demonstrates phantom data and
/// zero-sized markers (for more advanced use cases).
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }

    fn get(&self) -> &T {
        &self.value
    }

    fn into_inner(self) -> T {
        self.value
    }
}

fn main() {
    // Point with integers
    let ip = Point::new(5, 10);
    ip.show();
    println!("  ip = ({}, {})", ip.x, ip.y);

    // Point with floats
    let fp = Point::new(1.5, 3.14);
    fp.show();
    println!("  fp = ({}, {})", fp.x, fp.y);

    // Point with strings (works because String implements Display)
    let sp = Point::new("hello".to_string(), "world".to_string());
    sp.show();

    // Pair with mixed types
    let pair = Pair::new(42, 3.14);
    println!("pair = ({}, {})", pair.first, pair.second);
    let (n, f) = pair.into_tuple();
    println!("destructured: n={n}, f={f}");

    // Wrapper
    let w = Wrapper::new("Rust generics");
    println!("wrapper.get() = {}", w.get());
    let s = w.into_inner();
    println!("wrapper.into_inner() = {s}");
}
