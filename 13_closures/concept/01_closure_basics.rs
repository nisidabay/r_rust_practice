// 01_closure_basics.rs — |args| body syntax, type inference, calling closures
//
// Closures are anonymous functions that can capture variables.
// The simplest form: |params| expression

fn main() {
    // Basic closure: takes two i32, returns i32 (types inferred)
    let add = |a, b| a + b;
    println!("add(3, 5) = {}", add(3, 5));

    // Closure with explicit type annotations (optional, for clarity)
    let multiply: fn(i32, i32) -> i32 = |x: i32, y: i32| -> i32 { x * y };
    println!("multiply(4, 7) = {}", multiply(4, 7));

    // Closure with a block body (multiple statements)
    let complex = |x: i32| -> i32 {
        let y = x + 1;
        let z = y * 2;
        z - 3
    };
    println!("complex(10) = {}", complex(10));

    // Closure that captures nothing — behaves exactly like a function
    let no_capture = |s: &str| format!("Hello, {}!", s);
    println!("{}", no_capture("World"));

    // Type inference means closures adapt to how they're used
    let infer = |x| x * 2;
    println!("infer(5) = {}", infer(5));  // x is i32
    // println!("infer(3.14) = {}", infer(3.14));  // ERROR: already inferred as i32

    // Closures can be passed to functions
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    println!("Closures: inline functions with type inference.");
}
