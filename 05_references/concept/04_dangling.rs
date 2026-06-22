/*
 * 04_dangling.rs — Practical Rust
 *
 * Question: What is a dangling reference and how does Rust prevent it?
 *
 * A dangling reference points to memory that has been freed.
 * Rust's borrow checker catches these at COMPILE TIME.
 * The compiler error says: "missing lifetime specifier" or
 * "borrowed value does not live long enough".
 *
 * Fix: Return the OWNED value instead of a reference to something
 * that will be dropped.
 */

fn main() {
    // This works — we return an owned String
    let result = get_message_owned();
    println!("result: {}", result);

    // Demonstration of why dangling is impossible in safe Rust:
    let s = String::from("hello");
    let first = first_char(&s);
    println!("first char: {}", first);
}

// BAD: This would create a dangling reference
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // COMPILE ERROR: returns reference to local variable
// } // s is dropped here, reference would dangle

// FIX: Return the owned value directly
fn get_message_owned() -> String {
    let s = String::from("hello");
    s // ownership moves to caller, no dangling
}

// This is fine — we return a reference to data that outlives the function
fn first_char(s: &str) -> &str {
    if s.is_empty() {
        ""
    } else {
        &s[0..1]
    }
}
