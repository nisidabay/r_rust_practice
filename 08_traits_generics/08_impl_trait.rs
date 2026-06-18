// 08_impl_trait.rs
// impl Trait — concise syntax for "some type that implements a trait".
//
// `impl Trait` can appear in argument position (anonymous generic parameter)
// and return position (opaque return type). It's syntactic sugar that avoids
// naming complex types or writing verbose generics.
//
// Run: rustc --edition 2021 08_impl_trait.rs && ./08_impl_trait

use std::fmt::Display;
use std::iter::Iterator;

// ── impl Trait in argument position ──────────────────────────────────────────
//
// `fn foo(x: impl Display)` is sugar for `fn foo<T: Display>(x: T)`.
// Use it when you don't need turbofish or to name the type elsewhere.

/// Greets anything that implements Display.
fn greet(name: impl Display) {
    println!("Hello, {name}!");
}

/// Works with any two Display-able things.
fn announce(a: impl Display, b: impl Display) {
    println!("{a} and {b} — what a pair!");
}

// ── impl Trait in return position ────────────────────────────────────────────
//
// `fn foo() -> impl Trait` returns "some concrete type that implements Trait".
// The caller can only use Trait methods on the result. The concrete type is
// determined by the function body and is opaque to the caller.

/// Returns something Display-able. The caller knows only that it implements Display.
fn make_greeting(name: &str) -> impl Display + '_ {
    format!("Greetings, {name}!") // returns String
}

/// Returns a number as a Display-able value.
fn make_number(n: i32) -> impl Display {
    n // returns i32
}

// ── impl Trait with closures (the most common use case) ──────────────────────
//
// Closures have unique, unnameable types. `impl Trait` is the only way to
// return them from functions.

/// Returns a closure that adds n to its argument.
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

/// Returns a closure that multiplies by n.
fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

// ── impl Trait with iterators (very common) ──────────────────────────────────

/// Returns an iterator over even numbers up to limit.
fn evens_up_to(limit: u32) -> impl Iterator<Item = u32> {
    (0..=limit).filter(|x| x % 2 == 0)
}

/// Returns a transforming iterator.
fn square_odds(limit: u32) -> impl Iterator<Item = u32> {
    (0..=limit).filter(|x| x % 2 == 1).map(|x| x * x)
}

// ── impl Trait vs generics: when to use what ─────────────────────────────────
//
// Use generics <T: Trait> when:
//   - You need to name the type (e.g., turbofish, or same type in multiple args)
//   - You need additional trait bounds on the same type elsewhere
//   - You want the caller to explicitly specify the type
//
// Use impl Trait when:
//   - The exact type doesn't matter
//   - You're returning a closure, iterator, or other unnameable type
//   - You want simpler, more readable signatures

/// Generic version — same type T for both parameters (caller must use same type).
fn both_same<T: Display>(a: T, b: T) {
    println!("both_same: {a} and {b}");
}

/// impl Trait version — two independent parameters (can be different types).
fn both_any(a: impl Display, b: impl Display) {
    println!("both_any: {a} and {b}");
}

fn main() {
    // Argument position
    greet("world"); // &str implements Display
    greet(42); // i32 implements Display
    greet(3.14); // f64 implements Display
    announce("hello", 100);

    // Return position
    let msg = make_greeting("Alice");
    println!("{msg}");

    let n = make_number(255);
    println!("{n}");

    // Closures
    let add5 = make_adder(5);
    let mul3 = make_multiplier(3);
    println!("add5(10) = {}", add5(10));
    println!("mul3(10) = {}", mul3(10));

    // Iterators
    println!("Evens up to 10:");
    for e in evens_up_to(10) {
        print!("{e} ");
    }
    println!();

    println!("Squares of odds up to 10:");
    for s in square_odds(10) {
        print!("{s} ");
    }
    println!();

    // impl Trait vs generics
    both_same(10, 20); // OK: both i32
    // both_same(10, "hello"); // Error: type mismatch
    both_any(10, "hello"); // OK: different types
}
