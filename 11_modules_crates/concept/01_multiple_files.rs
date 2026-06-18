#!/usr/bin/env rustc
// HOW DO YOU GROW YOUR PROGRAM WITHOUT DROWNING IN ONE FILE?
//
// One Question: How do you grow your program without drowning in one file?
// Quick Start: rustc --edition 2021 01_multiple_files.rs util.rs && ./01_multiple_files
// Learning Path: multi-file → modules → cargo → workspaces
// Common Pattern: split helper functions into separate .rs files, compile together
// Now Build Your Own: Split a small program into main.rs + helpers.rs

/// This file demonstrates splitting code across multiple .rs files.
/// We compile both files together: `rustc --edition 2021 main.rs util.rs`
///
/// main.rs (this file) contains the entry point.
/// util.rs contains helper functions.

// Declare the external module (the other .rs file)
#[path = "util.rs"]
mod util;

fn main() {
    let name = "Rustacean";
    println!("{}", util::greet(name));
    println!("{}", util::farewell(name));

    let result = util::add(10, 20);
    println!("10 + 20 = {}", result);
}
