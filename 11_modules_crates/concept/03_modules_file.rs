#!/usr/bin/env rustc
// HOW DO YOU GROW YOUR PROGRAM WITHOUT DROWNING IN ONE FILE?
//
// One Question: How do you grow your program without drowning in one file?
// Quick Start: rustc --edition 2021 03_modules_file.rs && ./03_modules_file
// Learning Path: multi-file → modules → cargo → workspaces
// Common Pattern: `mod my_module;` loads `my_module.rs` from the same directory
// Now Build Your Own: Extract a module into its own .rs file

/// Demonstrates loading a module from a separate file.
/// `mod greetings;` tells Rust to find `greetings.rs` in the same directory.

mod greetings;

fn main() {
    println!("{}", greetings::hello("Bob"));
    println!("{}", greetings::goodbye("Bob"));
}
