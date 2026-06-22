/*
 * 02_move.rs — Practical Rust
 *
 * Question: What happens when I assign one variable to another?
 *
 * Move semantics: When you assign (or pass) a value that doesn't implement Copy,
 * ownership MOVES to the new variable. The old variable is INVALIDATED.
 * This prevents double-free bugs at compile time.
 *
 * Affects: String, Vec, and most heap-allocated types.
 */

fn main() {
    // String does NOT implement Copy — it uses move semantics
    let s1 = String::from("hello");
    let s2 = s1; // MOVES ownership from s1 to s2

    // println!("{}", s1); // COMPILE ERROR: s1 has been moved, value used here after move
    println!("s2 = {}", s2); // s2 owns the string now

    // Vec also moves
    let v1 = vec![1, 2, 3];
    let v2 = v1; // moves

    // println!("{:?}", v1); // COMPILE ERROR: v1 invalid after move
    println!("v2 = {:?}", v2);

    // Move in a function call
    let name = String::from("Alice");
    process_name(name);
    // println!("{}", name); // COMPILE ERROR: moved into function

    // But i32 does implement Copy — no move
    let x = 5;
    let y = x; // x is COPIED, not moved
    println!("x = {}, y = {}", x, y); // both valid!
}

fn process_name(n: String) {
    println!("Processing: {}", n);
} // n is dropped here
