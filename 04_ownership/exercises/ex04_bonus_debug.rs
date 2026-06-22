/*
 * ex04_bonus_debug.rs — Bonus Exercise
 *
 * Task: This file has intentional OWNERSHIP errors.
 *       Fix them by writing correct code BELOW each section,
 *       with comments explaining each fix.
 *
 * Run: ./ex04_bonus_debug  (will print success messages when all fixes are in)
 */

use std::io::{self, BufRead};

// === Section 1: Use after move ===
// BUG: s1 is used after its value moved to s2
// FIX: Use .clone() if both are needed, or just use s2
fn section1() {
    println!("=== Section 1: Use after move ===");

    let s1 = String::from("hello");

    // FIX 1: Clone s1 so both variables own valid data
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2); // Both valid now!
}

// === Section 2: Moving into function ===
// BUG: s is moved into take_ownership, then used again
// FIX: Pass a reference instead, or clone before passing
fn section2() {
    println!("=== Section 2: Moving into function === ");

    let s = String::from("world");

    // FIX 2: Pass a reference (&s) so ownership is borrowed, not moved
    take_ownership(&s);

    println!("s still works: {}", s); // Valid because we borrowed!
}

fn take_ownership(s: &str) {
    println!("took: {}", s);
}

// === Section 3: Double move from Vec ===
// BUG: first element is moved out of vec, leaving vec partially invalid
// FIX: Use swap_remove or iterate by reference
fn section3() {
    println!("=== Section 3: Double move from Vec ===");

    let mut names = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Charlie"),
    ];

    // FIX 3: Use swap_remove to remove an element without invalidating the whole vec
    let first = names.swap_remove(0); // Removes index 0, swaps last element in
    println!("Removed: {}", first);
    println!("Remaining: {:?}", names);
    // names is still fully valid here
}

// === Section 4: Ownership in a loop ===
// BUG: String is consumed in each iteration, can't reuse
// FIX: Clone or use reference in the loop
fn section4() {
    println!("=== Section 4: Ownership in a loop ===");

    let data = String::from("hello world foo");

    // FIX 4: Iterate by reference with split_whitespace()
    for word in data.split_whitespace() {
        println!("word: {}", word);
    }

    // Can still use data because we borrowed it
    println!("data still valid: {}", data);
}

fn main() {
    section1();
    section2();
    section3();
    section4();

    println!("\nAll sections fixed! No ownership bugs remaining.");
}
