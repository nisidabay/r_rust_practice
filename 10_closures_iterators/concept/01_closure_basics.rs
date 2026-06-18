// 01_closure_basics — |args| body syntax, type inference
//
// Closures are anonymous functions that can capture variables from
// their surrounding scope. They use a lightweight |args| body syntax.
// Unlike functions, closures can infer their parameter and return types.

fn main() {
    // --- 1. Basic closure syntax ---
    // A closure that takes no arguments and returns nothing.
    let greet = || {
        println!("Hello from a closure!");
    };
    greet();

    // --- 2. Single-expression closure (no braces needed) ---
    let double = |x| x * 2;
    println!("double(5) = {}", double(5));

    // --- 3. Multi-parameter closure ---
    let add = |a, b| a + b;
    println!("add(3, 7) = {}", add(3, 7));

    // --- 4. Type inference — Rust infers types from first use ---
    let square = |x| x * x; // type inferred as i32 because we call with i32
    println!("square(4) = {}", square(4));
    // square(4.0); // ERROR: once inferred as i32, cannot use f64

    // --- 5. Explicit type annotation ---
    let square_f64 = |x: f64| -> f64 { x * x };
    println!("square_f64(2.5) = {}", square_f64(2.5));

    // --- 6. Closures as values (stored in variables) ---
    let is_even = |x: i32| -> bool { x % 2 == 0 };
    println!("is_even(7) = {}, is_even(10) = {}", is_even(7), is_even(10));

    // --- 7. Calling a closure stored in a variable ---
    let multiply = |a: i32, b: i32| { a * b };
    let result = multiply(6, 7);
    println!("multiply(6, 7) = {}", result);

    println!("\nAll closure basics demonstrated.");
}
