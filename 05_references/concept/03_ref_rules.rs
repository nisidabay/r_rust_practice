/*
 * 03_ref_rules.rs — Practical Rust
 *
 * Question: What happens when I break the reference rules?
 *
 * Rule: At any given time, you can have EITHER
 *   - one mutable reference, OR
 *   - any number of immutable references
 * These compile errors are PROTECTING you from data races.
 * Fix by using scoping {} to limit how long a borrow lives.
 */

fn main() {
    // === ERROR 1: Two mutable references to same data ===
    let mut s = String::from("hello");
    // FIX: Use scoping
    {
        let r1 = &mut s;
        r1.push_str(" world");
    } // r1's borrow ends here
    {
        let r2 = &mut s;
        r2.push_str("!!!");
    } // r2's borrow ends here
    println!("s = {}", s);

    // === ERROR 2: Immutable + mutable reference ===
    let mut data = vec![1, 2, 3];
    // FIX: Use the immutable ref first, then the mutable ref after
    let first = &data[0]; // immutable borrow starts
    println!("first: {}", first); // used here
    // immutable borrow ends after last use (NLL — Non-Lexical Lifetimes)

    let mut_ref = &mut data; // mutable borrow — OK now
    mut_ref.push(4);
    println!("data: {:?}", data);

    // === ERROR 3: Using variable while mutable borrow is active ===
    let mut n = 10;
    let r = &mut n;
    *r += 5;
    // println!("n = {}", n); // COMPILE ERROR: can't use n while mutably borrowed
    println!("r = {}", r); // use r instead
    // After r's last use, n is available again
    println!("n = {}", n); // OK now

    // === Scoping pattern to avoid issues ===
    let mut items = vec![1, 2, 3, 4, 5];
    {
        let slice = &mut items[1..4];
        for item in slice.iter_mut() {
            *item *= 10;
        }
    } // slice borrow ends here
    println!("items after scoped mutation: {:?}", items);
}
