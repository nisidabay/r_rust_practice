/// Exercise 3: Create a small library + binary that uses it
///
/// This exercise involves TWO files:
/// - ex03_library.rs — the library crate root
/// - ex03_library_vs_binary.rs — the binary (this file)
///
/// Your task:
/// 1. Create `ex03_library.rs` with a `pub fn is_even(n: i32) -> bool` function.
/// 2. Create a `pub mod stats` inside the library with a `pub fn mean(values: &[f64]) -> Option<f64>`.
/// 3. In THIS file, declare `mod ex03_library;` and use its functions.
///
/// Compile: rustc --edition 2021 ex03_library_vs_binary.rs ex03_library.rs && ./ex03_library_vs_binary

// TODO: Declare the external library module

fn main() {
    // TODO: Use ex03_library::is_even and ex03_library::stats::mean
    println!("Replace this with your solution!");
}
