// 03_scalar_types — integers, floats, bool, char
// Rust has four scalar types: integers, floats, booleans, characters.
// Sizes are explicit: i32, u64, f64 — no implicit widening/narrowing.
// Use `_` as a digit separator for readability.

fn main() {
    // ----- integers -----
    let signed: i32 = -42;          // 32-bit signed, default integer type
    let unsigned: u64 = 1_000_000;  // 64-bit unsigned
    let byte_literal: u8 = b'A';    // 65 — ASCII byte literal

    println!("signed: {signed}, unsigned: {unsigned}, byte: {byte_literal}");

    // ----- floats -----
    let pi: f64 = 3.141592653589793; // f64 is the default float type (double precision)
    let approx: f32 = 3.14;          // f32 is single precision

    println!("pi (f64) = {pi}, approx (f32) = {approx}");

    // ----- booleans -----
    let is_rust_fun: bool = true;
    let is_hard: bool = false;
    println!("fun? {is_rust_fun}, hard? {is_hard}");

    // ----- char -----
    // char is 4 bytes (Unicode scalar value), NOT 1 byte like C.
    let letter: char = 'R';
    let emoji: char = '🦀';
    println!("letter: {letter}, emoji: {emoji}");

    // ----- arithmetic with different types needs explicit cast -----
    let a: i32 = 10;
    let b: f64 = 3.0;
    // println!("{}", a + b); // COMPILE ERROR: mismatched types
    let result = a as f64 + b;     // cast i32 -> f64
    println!("{a} + {b} = {result}");
}
