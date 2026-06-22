/*
 * 02_mutable_ref.rs — Practical Rust
 *
 * Question: How do I change data through a reference?
 *
 * &mut T = mutable reference. You can READ and WRITE through it.
 * Exclusive writer rule: only ONE mutable reference at a time.
 * No other references (mutable OR immutable) may exist simultaneously.
 * This prevents data races at compile time.
 */

fn main() {
    let mut s = String::from("hello");

    // Create a mutable reference
    let r = &mut s;
    r.push_str(" world"); // modify through the reference
    println!("r: {}", r);

    // r's borrow ends here — we can use s again
    println!("s: {}", s);

    // Exclusive writer rule in action:
    let mut x = 42;
    {
        let mx: &mut i32 = &mut x;
        *mx += 10;
        println!("mx: {}", mx);
        // mx goes out of scope, borrow ends
    }
    println!("x after: {}", x); // x is accessible again

    // Cannot have both mutable and immutable references at same time
    let mut data = String::from("test");
    // let r1 = &data;      // immutable borrow
    // let r2 = &mut data;  // COMPILE ERROR: can't borrow as mutable because immutable exist
    // println!("{} {}", r1, r2);

    // Scoping solves this — borrows end at different times
    let r1 = &data; // immutable borrow
    println!("r1: {}", r1); // r1 used and its borrow ends
    let r2 = &mut data; // mutable borrow — OK, r1 is done
    r2.push_str("ing");
    println!("r2: {}", r2);
}
