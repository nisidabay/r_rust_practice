/*
 * 01_if_else.rs — Practical Rust
 *
 * Question: How do I use if/else in Rust?
 *
 * if/else is an EXPRESSION — it returns a value.
 * No ternary operator (?:) needed — use if/else directly.
 */

fn main() {
    let x = 10;

    // Basic if/else
    if x > 5 {
        println!("{} is greater than 5", x);
    } else {
        println!("{} is not greater than 5", x);
    }

    // else if chain
    let score = 85;
    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B"); // this runs
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }

    // if as an expression — NO ternary operator needed
    let age = 20;
    let status = if age >= 18 { "adult" } else { "minor" };
    println!("{} is an {}", age, status);

    // Both branches must be the same type
    let is_even = if x % 2 == 0 { true } else { false };
    println!("{} is even: {}", x, is_even);

    // Inline if in println!
    println!("{} is {}", x, if x > 5 { "big" } else { "small" });

    // if with let and early return (no else needed with return/break)
    let nums = [1, 2, 3];
    if nums.is_empty() {
        println!("empty array");
        return; // no else needed
    }
    println!("array has {} elements", nums.len());
}
