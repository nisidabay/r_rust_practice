/*
 * 01_ownership_intro.rs — Practical Rust
 *
 * Question: What is ownership?
 *
 * Three rules of ownership:
 * 1. Each value in Rust has an owner.
 * 2. There can only be one owner at a time.
 * 3. When the owner goes out of scope, the value is dropped.
 *
 * Scope = where a variable is valid. When it ends (}), drop() is called.
 */

fn main() {
    // Rule 1: Each value has one owner
    let s1 = String::from("hello"); // s1 owns "hello"
    println!("s1 = {}", s1);

    // Scope demonstration
    {
        let inner = String::from("inside");
        println!("inner = {}", inner);
    } // inner goes out of scope → dropped here
    // println!("inner = {}", inner); // COMPILE ERROR: inner no longer exists

    // Ownership at function boundaries
    let s2 = String::from("world");
    take_ownership(s2);
    // println!("s2 = {}", s2); // COMPILE ERROR: s2 moved to function, dropped at end of take_ownership

    // Returning ownership
    let s3 = give_ownership();
    println!("s3 = {}", s3);
}

fn take_ownership(s: String) {
    println!("take_ownership got: {}", s);
} // s is dropped here

fn give_ownership() -> String {
    let result = String::from("returned");
    result // ownership transfers to caller
}
