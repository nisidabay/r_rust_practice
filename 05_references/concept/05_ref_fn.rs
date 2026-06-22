/*
 * 05_ref_fn.rs — Practical Rust
 *
 * Question: How do I write functions that take references?
 *
 * Functions that take &T can read the data.
 * Functions that take &mut T can read and modify the data.
 * Use &T when you only need to look. Use &mut T when you need to change.
 * A classic example: swap two values using &mut references.
 */

fn main() {
    // Swap example
    let mut a = 5;
    let mut b = 10;

    println!("Before: a = {}, b = {}", a, b);
    swap(&mut a, &mut b);
    println!("After:  a = {}, b = {}", a, b);

    // Read a value through &T
    let msg = String::from("hello world");
    let len = string_length(&msg);
    println!("'{}' has {} chars", msg, len); // msg still valid

    // Modify through &mut T
    let mut greeting = String::from("hello");
    add_exclamation(&mut greeting);
    println!("greeting: {}", greeting);

    // Multiple &T is fine
    let x = 42;
    let y = 100;
    let sum = add_refs(&x, &y);
    println!("{} + {} = {}", x, y, sum);
}

fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn string_length(s: &String) -> usize {
    s.len() // auto-deref
}

fn add_exclamation(s: &mut String) {
    s.push_str("!");
}

fn add_refs(a: &i32, b: &i32) -> i32 {
    *a + *b // dereference to add values
}
