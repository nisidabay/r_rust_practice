// References (&T) let you borrow data without taking ownership.
// A reference is non-owning — it points to a value owned by someone else.
// Immutable references (&T) are Copy and can have many readers simultaneously.

fn main() {
    // === BASIC IMMUTABLE REFERENCES ===

    let s: String = String::from("hello");

    // &s creates a reference to s — borrows without taking ownership
    let len: usize = calculate_length(&s);

    // s is still valid because we only lent it out!
    println!("1. '{}' has length {} — s still owned here", s, len);

    // === MULTIPLE IMMUTABLE REFERENCES ===
    // You can have MANY immutable references to the same data

    let data: String = String::from("shared data");
    let r1: &String = &data;
    let r2: &String = &data;
    let r3: &String = &data;

    println!("2. r1: {}, r2: {}, r3: {}", r1, r2, r3);
    // All three references are valid simultaneously
    // data is still the owner — it hasn't moved

    // === DEREFERENCING ===
    // Use * to follow a reference back to the value

    let x: i32 = 10;
    let rx: &i32 = &x;
    // Comparing references compares the value behind them automatically
    println!("3. rx points to {}", rx);
    // Explicit dereference: *rx gives the i32 value
    assert_eq!(*rx, 10);

    // === REFERENCES WITH COPY TYPES ===
    // You can reference Copy types too, even though it's unnecessary

    let n: i32 = 100;
    let rn: &i32 = &n;
    println!("4. n={}, rn={} — both accessible", n, rn);
    // n is Copy, so we can still use n even without the reference

    // === REFERENCE RULES ===
    // 1. At any given time, you can have EITHER:
    //    - One mutable reference (&mut T)
    //    - Any number of immutable references (&T)
    // 2. References must always be valid (no dangling pointers)

    // This is valid: many immutable references
    let text: String = String::from("rules");
    let a: &String = &text;
    let b: &String = &text;
    let c: &String = &text;
    println!("5. {} {} {} — all immutable refs to same data", a, b, c);
    // All good — we only have immutable references

    println!("6. ✓ Immutable references demonstrated");

    // text is dropped here — but only after all references are done with it
    // The compiler ensures references don't outlive their owner
}

// &String means "a reference to a String" — borrows, doesn't own
fn calculate_length(s: &String) -> usize {
    // s is a reference — we can read the data but not modify it
    s.len()
    // s goes out of scope here, but since it's just a reference,
    // the underlying String is NOT dropped
}
