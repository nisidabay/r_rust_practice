/*
 * 04_for.rs — Practical Rust
 *
 * Question: How do I use for loops in Rust?
 *
 * for i in 0..n — range (exclusive end)
 * for i in 0..=n — inclusive range
 * for item in collection — iterate over elements
 * for (i, item) in iter.enumerate() — get index + element
 */

fn main() {
    // Range loop: 0..5 = 0, 1, 2, 3, 4 (exclusive end)
    println!("Range 0..5:");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!("");

    // Inclusive range: 0..=5 = 0, 1, 2, 3, 4, 5
    println!("Inclusive range 0..=5:");
    for i in 0..=5 {
        print!("{} ", i);
    }
    println!("");

    // Iterating over a vector (by reference)
    let numbers = vec![10, 20, 30, 40];
    println!("Iterating vec:");
    for n in &numbers {
        print!("{} ", n);
    }
    println!("");

    // Iterating with mutable reference — modify each element
    let mut values = vec![1, 2, 3, 4];
    for v in &mut values {
        *v *= 2; // dereference to modify
    }
    println!("Doubled: {:?}", values);

    // enumerate() — get index and value
    let fruits = vec!["apple", "banana", "cherry"];
    println!("Enumerated fruits:");
    for (i, fruit) in fruits.iter().enumerate() {
        println!("  {}: {}", i, fruit);
    }

    // for over a string's chars
    let word = "hello";
    println!("Chars in '{}':", word);
    for c in word.chars() {
        print!("{} ", c);
    }
    println!("");

    // for over a range with step (using .step_by())
    for i in (0..=10).step_by(2) {
        print!("{} ", i);
    }
    println!(" (even numbers)");
}
