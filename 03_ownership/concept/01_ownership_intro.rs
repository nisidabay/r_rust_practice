// Rust's ownership: each value has exactly ONE owner.
// When the owner goes out of scope, the value is dropped (memory freed).
// No GC, no manual free — ownership rules guarantee safety at compile time.

fn main() {
    // THE THREE RULES OF OWNERSHIP
    // 1. Each value has exactly ONE owner
    // 2. Ownership can be transferred (moved) from one owner to another
    // 3. When the owner goes out of scope, the value is dropped

    // Rule 1: s is the owner of the String "hello"
    let s = String::from("hello");
    println!("1. s owns: {}", s);

    // Rule 3 in action: scope block — value dropped when scope ends
    {
        let inner = String::from("temporary");
        println!("2. inner exists here: {}", inner);
    } // inner goes out of scope -> drop() called -> memory freed
    // println!("{}", inner); // COMPILE ERROR: inner no longer exists

    // Stack vs Heap intro:
    // i32 lives entirely on the stack — small, fixed size, copied cheaply
    let x: i32 = 42;
    let y: i32 = x; // x is COPIED (it's a Copy type); both x and y are valid
    println!("3. x={}, y={} — both valid because i32 is Copy", x, y);

    // String lives partly on heap — variable size, must be managed
    let s1 = String::from("heap data");
    let s2 = s1; // s1 is MOVED to s2, NOT copied
    // println!("{}", s1); // COMPILE ERROR: s1 no longer owns the data
    println!("4. s2 owns after move: {}", s2);

    // Demonstrate scope-based dropping
    let outer = String::from("outer");
    {
        let inner = String::from("inner");
        println!("5. Both alive: {} and {}", outer, inner);
    } // inner dropped here (heap freed)
    println!("6. outer still alive: {}", outer);
    // outer dropped here (heap freed)

    // Key insight: strings, vectors, and other heap-allocated types
    // follow move semantics. Simple scalar types (integers, bools, chars)
    // implement Copy and are automatically duplicated.
    println!("7. ✓ Ownership rules demonstrated");
}
