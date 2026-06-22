/*
 * 05_constants.rs — Practical Rust
 *
 * Question: How do I use constants in Rust?
 *
 * const = compile-time constant, always inline, no memory address
 * let = runtime variable, can be shadowed, has a memory address
 * const fn = function that runs at compile time, callable in const contexts
 */

// const is evaluated at compile time — value must be known before the program runs
// Naming convention: SCREAMING_SNAKE_CASE
const MAX_PLAYERS: u32 = 8;
const PI: f64 = 3.1415926535;
const GREETING: &str = "Hello from const";

// const fn — a function guaranteed to be evaluable at compile time
// Only a subset of Rust is allowed: no dynamic allocation, no I/O, etc.
const fn add(x: i32, y: i32) -> i32 {
    x + y // simple operations work
}

// const fn can call other const fns
const fn double(x: i32) -> i32 {
    add(x, x)
}

// Consts can use const fn results
const COMPUTED: i32 = add(10, 20);
const DOUBLED: i32 = double(5);

fn main() {
    // Using const values
    println!("MAX_PLAYERS = {}", MAX_PLAYERS);
    println!("PI = {:.10}", PI);
    println!("{}", GREETING);

    // const vs let — let is runtime, can shadow
    let x = 10;
    println!("let x = {}", x);
    let x = "now I'm a string"; // shadowing changes type
    println!("shadowed x = {}", x);

    // const can't be shadowed (it's not a variable)
    // Uncommenting this won't compile:
    // const MAX_PLAYERS: u32 = 10; // ERROR: duplicate definition

    // const fn results used at runtime
    println!("const fn add(10, 20) = {}", COMPUTED);
    println!("const fn double(5) = {}", DOUBLED);

    // const fn also works in regular (runtime) contexts
    let runtime_sum = add(3, 4);
    println!("const fn called at runtime: add(3, 4) = {}", runtime_sum);

    // Unlike const, static variables have a fixed memory address
    // (not covered in detail here, but good to know exists)
    static APP_NAME: &str = "Rust Practice";
    println!("static: {}", APP_NAME);
}
