// 01_unsafe_intro.rs — The `unsafe` keyword
//
// What do you do when you need to break the rules?
//
// The `unsafe` keyword in Rust does NOT turn off the borrow checker.
// It enables five specific abilities that are normally forbidden:
//
// 1. Dereference a raw pointer (*const T / *mut T)
// 2. Call an unsafe function or method
// 3. Access or modify a mutable static variable
// 4. Implement an unsafe trait
// 5. Access fields of a `union`
//
// IMPORTANT: unsafe doesn't mean "this code is always dangerous."
// It means "the compiler cannot prove this code is safe — YOU must."
// The responsibility shifts from the type system to the programmer.

fn main() {
    println!("=== Unsafe Intro ===");
    println!("The `unsafe` keyword enables 5 superpowers:");
    println!("  1. Dereference a raw pointer");
    println!("  2. Call unsafe functions");
    println!("  3. Access mutable statics");
    println!("  4. Implement unsafe traits");
    println!("  5. Access union fields");
    println!();

    // A tiny unsafe block to dereference a raw pointer.
    // This shows the syntax — we'll dive deeper in later files.
    let x: i32 = 42;
    let r: *const i32 = &x as *const i32;

    unsafe {
        println!("Dereferencing a raw pointer: *r = {}", *r);
    }

    println!();
    println!("Key principle: `unsafe` does NOT disable the borrow checker.");
    println!("It gives you five extra tools — use them wisely.");
    println!("With great power comes great responsibility (to uphold safety contracts).");
}
