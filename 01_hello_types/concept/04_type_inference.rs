// 04_type_inference — how Rust infers types
// Rust infers types from context: literals, operations, and usage.
// Add explicit annotations when inference is ambiguous (e.g., parse, collect).
// `let x = ...` without annotation triggers inference from the RHS.

fn main() {
    // Rust infers i32 for integer literals (default integer type).
    let x = 42;             // i32
    let y = 3.14;           // f64 — default float type
    let z = true;           // bool
    println!("x={x}, y={y}, z={z}");

    // Inference through operations: result type follows operand types.
    let sum = x + 10;       // i32 + i32 -> i32
    println!("sum = {sum}");

    // Inference through usage: Rust watches how a value is used.
    let guess = "42".parse::<i32>().expect("not a number");
    // Without ::<i32> (turbofish), we'd need: let guess: i32 = ...
    println!("guess = {guess}");

    // Type annotation clarifies when inference would be ambiguous.
    let pi: f64 = 3.1415;   // Without :f64, this would also be f64, but explicit is clear
    let bits: u64 = 64;     // Without :u64, Rust would default to i32
    println!("pi={pi}, bits={bits}");

    // Shared references let Rust infer from usage too.
    let greeting = "hello";         // &str — string slice
    let greeting_ptr = &greeting;   // &&str — inferred from &greeting
    println!("greeting_ptr = {greeting_ptr}");

    // Key insight: Rust's type inference is local (function body, not cross-crate).
    // It's powerful but not Hindley-Milner — you still annotate function signatures.
}
