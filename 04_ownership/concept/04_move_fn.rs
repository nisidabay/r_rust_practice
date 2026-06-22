/*
 * 04_move_fn.rs — Practical Rust
 *
 * Question: How does ownership work when calling functions?
 *
 * Passing a non-Copy value to a function MOVES ownership to the function.
 * The function can return ownership back to the caller.
 * This is why we use references (next group) — constantly moving and
 * returning is tedious.
 */

fn main() {
    let s = String::from("hello");

    // Ownership moves into the function
    take(s);
    // println!("{}", s); // COMPILE ERROR: s moved into take()

    // But we can get ownership back
    let s2 = String::from("world");
    let s2 = give_back(s2); // moves in, moves back (rebinding with let)
    println!("s2 is back: {}", s2);

    // Multiple values: pass and return
    let s3 = String::from("foo");
    let s4 = String::from("bar");
    let (s3, s4, len) = calculate_length(s3, s4);
    println!("'{}' has {} chars, '{}' still valid", s3, len, s4);
}

fn take(s: String) {
    println!("took: {}", s);
} // s dropped here

fn give_back(s: String) -> String {
    println!("giving back: {}", s);
    s // ownership returned
}

fn calculate_length(s1: String, s2: String) -> (String, String, usize) {
    let len = s1.len() + s2.len();
    (s1, s2, len) // return both strings and the computed length
}
