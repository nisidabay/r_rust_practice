#!/usr/bin/env rustc
// HOW DO YOU GROW YOUR PROGRAM WITHOUT DROWNING IN ONE FILE?
//
// One Question: How do you grow your program without drowning in one file?
// Quick Start: rustc --edition 2021 02_modules_basics.rs && ./02_modules_basics
// Learning Path: multi-file → modules → cargo → workspaces
// Common Pattern: use `mod` to group related code within the same file
// Now Build Your Own: Inline a few modules to organize a small program

/// Demonstrates inline modules using the `mod` keyword.
/// Modules are defined directly inside this file.

// --- inline module: greetings ---
mod greetings {
    pub fn hello(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    pub fn goodbye(name: &str) -> String {
        format!("Goodbye, {}!", name)
    }

    // Private function — not accessible outside this module
    fn internal_only() {
        println!("This is internal to greetings");
    }
}

// --- inline module: math_ops ---
mod math_ops {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

fn main() {
    // Use functions from inline modules via their path
    println!("{}", greetings::hello("Alice"));
    println!("{}", greetings::goodbye("Alice"));

    println!("3 + 7 = {}", math_ops::add(3, 7));
    println!("4 * 5 = {}", math_ops::multiply(4, 5));

    // The following would NOT compile because `internal_only` is private:
    // greetings::internal_only();
}
