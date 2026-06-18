// 01_declarative_intro.rs — macro_rules! basics, matching patterns
//
// Q: "How do you write code that writes code?"
// A: Declarative macros (macro_rules!) let you define pattern-matched code
//    transformations that expand at compile time.
//
// This file introduces the core building blocks of Rust's declarative macros.
// Run with: rustc --edition 2021 01_declarative_intro.rs && ./01_declarative_intro

/// A minimal macro that matches a single expression and prints it.
/// The pattern `($x:expr)` means: match one expression token tree, bind it to $x.
macro_rules! say_hello_to {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

/// A macro that matches nothing (zero arguments).
macro_rules! greet {
    () => {
        println!("Hello, world!");
    };
}

/// A macro with multiple arms, chosen by pattern matching.
macro_rules! describe {
    () => {
        println!("Nothing to describe.");
    };
    ($x:expr) => {
        println!("You passed a single value: {}", $x);
    };
    ($x:expr, $y:expr) => {
        println!("You passed two values: {} and {}", $x, $y);
    };
}

/// A macro that matches a literal keyword token (not an expression).
macro_rules! make_pair {
    (pair $a:expr, $b:expr) => {
        ($a, $b)
    };
}

fn main() {
    println!("=== macro_rules! Basics ===");

    // Basic macro with expression argument
    say_hello_to!("Rust");

    // No-argument macro
    greet!();

    // Multi-arm macro
    describe!();
    describe!(42);
    describe!("foo", "bar");

    // Macro matching a keyword + expressions
    let p = make_pair!(pair "answer", 42);
    println!("Pair: {:?}", p);

    println!("\nKey takeaway: macro_rules! arms are pattern-matched at compile time.");
    println!("Each arm: (pattern) => (expansion)");
    println!("The expander finds the first matching arm and substitutes $vars.");
}
