/// Exercise 1: Split greet function into separate module, use from main
///
/// Your task:
/// 1. Create a module `greeter` (inline or in a separate file) with a `pub fn greet(name: &str) -> String`
///    that returns: "Hello, {name}! Welcome to Rust modules."
/// 2. Call `greeter::greet` from main and print the result.
/// 3. Add a private helper function `excited_greet` inside the module (not exported).
///
/// HINT: Use `mod greeter { ... }` for an inline module.
///
/// Run: rustc --edition 2021 ex01_greeter.rs && ./ex01_greeter

// TODO: Define the `greeter` module here with the greet function

fn main() {
    // TODO: Call greeter::greet("Alice") and print the result
    println!("Replace this with your solution!");
}

// Tests (do not modify)
#[test]
fn test_greet() {
    // Only compile-test: ensure the module compiles
}
