// 01_panic — What panic! does, unwinding, when to (not) use it
//
// panic! is Rust's "crash now" button. When called, it prints a failure
// message, unwinds the stack (running destructors), and exits the program.
//
// Use panic! for unrecoverable states: array index out of bounds, integer
// overflow in debug mode, unwrap on None where None is a logic bug.
// Do NOT use panic! for expected failures like bad user input or file not
// found — those should use Result.

fn main() {
    // --- 1. A controlled panic ---
    // Commented out so this file compiles and runs:
    // panic!("Something went terribly wrong!");

    // --- 2. Panic from an out-of-bounds index ---
    let v = vec![10, 20, 30];
    // Uncommenting the next line panics at runtime:
    // let x = v[99];
    // println!("x = {x}"); // never reached

    // --- 3. Safer access with .get() ---
    // .get() returns Option, so we handle absence without panicking.
    match v.get(99) {
        Some(val) => println!("Found: {val}"),
        None => println!("Index 99 is out of bounds (Vec has {} elements).", v.len()),
    }

    // --- 4. Integer overflow ---
    // In debug mode, overflow panics. In release, it wraps (two's complement).
    let big: u8 = 250;
    // let overflow = big + 10; // panics in debug mode
    let (wrapped, overflowed) = big.overflowing_add(10);
    println!(
        "250 + 10 = (wrapped: {}, overflowed: {})",
        wrapped, overflowed
    );

    // --- 5. assert! macros also panic ---
    let a = 2 + 2;
    assert!(a == 4, "Math is broken: {a} != 4");
    // assert_eq!(a, 5); // uncomment to panic

    // --- 6. When panic! is acceptable ---
    // In examples, prototypes, and tests — but replace with Result for
    // production code.
    fn divide_unchecked(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("Division by zero is undefined!");
        }
        a / b
    }

    // If we know by invariant that b is never zero, panic! is okay.
    println!("divide_unchecked(10, 2) = {}", divide_unchecked(10, 2));

    // --- 7. catch_unwind (advanced) ---
    // panic! normally kills the thread. You can "catch" it with
    // std::panic::catch_unwind, but this is rare and NOT recommended for
    // general error handling. Use Result instead.
    use std::panic;

    let result = panic::catch_unwind(|| {
        // If this line panics, we catch it:
        panic!("oops!");
    });

    match result {
        Ok(_) => println!("Completed normally."),
        Err(e) => println!("Caught a panic: {:?}", e.downcast_ref::<&str>()),
    }

    println!("--- 01_panic done ---");
}
