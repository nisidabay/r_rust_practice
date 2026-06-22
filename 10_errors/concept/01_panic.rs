fn main() {
    // panic! — the "crash" button. Use it when your program can't continue.
    // In C, you'd segfault or abort(). In Rust, panic! unwinds the stack.

    // --- Basic panic ---
    // Runs, prints "panicked at ...", and unwinds (calling destructors)
    // println!("About to panic...");
    // panic!("Something went wrong!");  // uncomment to see the panic
    // println!("This never runs");

    // --- Panic from out-of-bounds access ---
    let v = vec![1, 2, 3];
    // v[100];  // panics with: index out of bounds: len 3, index 100

    // --- Panic from unwrap on None ---
    let x: Option<i32> = None;
    // x.unwrap();  // panics: called `Option::unwrap()` on a `None` value

    // --- Panic from unwrap on Err ---
    let y: Result<i32, &str> = Err("oops");
    // y.unwrap();  // panics: called `Result::unwrap()` on an `Err` value

    // --- Controlled panic (for demonstration only) ---
    // In real code: panic! is for unrecoverable states. Use Result for recoverable errors.

    // Demonstrate what a panic looks like WITHOUT actually panicking
    // (The commented lines above show all the ways panic! happens)
    println!("Five ways to panic in Rust:");
    println!("  1. panic!() — explicit crash");
    println!("  2. vec[index] — out of bounds");
    println!("  3. Option::unwrap() on None");
    println!("  4. Result::unwrap() on Err");
    println!("  5. expect() on None/Err (same as unwrap but with a message)");
    println!("");
    println!("Panic = unwind. Destructors run. The program stops.");
    println!("Use `panic = \"abort\"` in Cargo.toml for embedded/constrained targets.");
}
