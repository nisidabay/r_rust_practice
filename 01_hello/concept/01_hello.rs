/*
 * 01_hello.rs — Practical Rust
 *
 * Question: How do I print something?
 *
 * Use println!() — a macro that prints text + newline.
 *   {}  — placeholder for any Display type
 *   {:?} — debug placeholder (for types that don't implement Display)
 *   {N} — positional argument
 *
 * Use print!() to print WITHOUT a newline.
 */

fn main() {
    // Basic string literal
    println!("Hello, world!");

    // {} placeholder — works for most built-in types
    println!("The answer is {}", 42);

    // Multiple placeholders
    println!("{} is {} years old", "Alice", 30);

    // Positional placeholders — reuse arguments
    println!("{0} + {0} = {1}", 2, 4);

    // Debug format {:?} — useful for arrays, enums, etc.
    println!("Debug: {:?}", (1, "hello", 3.14));

    // print! (no newline) + explicit newline
    print!("No newline here...");
    println!(" now there is one.");
}
