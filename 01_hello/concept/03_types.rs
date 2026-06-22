/*
 * 03_types.rs — Practical Rust
 *
 * Question: What kinds of data can I store?
 *
 * Scalar types (single values):
 *   i8..i128, u8..u128, isize, usize — integers
 *   f32, f64                    — floats
 *   bool                        — true / false
 *   char                        — 4-byte Unicode ('' quotes)
 *
 * Compound types (multiple values):
 *   (type, type, ...)           — tuple
 *   [type; N]                   — fixed-size array
 *
 * Rust INFERS types from context. Annotate when needed: let x: i32 = 5;
 */

fn main() {
    // --- Integers ---
    let a: i32 = -100;          // i32 is default for signed
    let b: u64 = 300;           // unsigned
    let c = 42;                 // inferred as i32 (default)

    println!("i32={}, u64={}, inferred={}", a, b, c);

    // --- Floats ---
    let pi: f64 = 3.1415926535; // f64 is default (double precision)
    let approx: f32 = 3.14;     // f32 (single precision)
    println!("f64={:.4}, f32={:.2}", pi, approx);

    // --- Boolean ---
    let is_rust_fun = true;
    let is_hard: bool = false;
    println!("fun={}, hard={}", is_rust_fun, is_hard);

    // --- Char (4 bytes, Unicode) ---
    let letter = 'A';
    let emoji = '🚀';
    let espanol = 'ñ';
    println!("letter={}, emoji={}, espanol={}", letter, emoji, espanol);

    // --- Tuple ---
    let tup: (i32, f64, char) = (42, 3.14, 'R');
    // Destructuring
    let (x, y, z) = tup;
    println!("tuple: x={}, y={}, z={}", x, y, z);
    // Direct access with .0, .1, .2
    println!("tup.0={}, tup.1={}", tup.0, tup.1);

    // --- Array (fixed size, same type) ---
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    println!("arr[0]={}, arr[2]={}", arr[0], arr[2]);
    // Shortcut: [value; count]
    let zeros = [0; 3];         // [0, 0, 0]
    println!("zeros: {:?}", zeros);

    // --- Type inference in action ---
    let guess = "42".parse::<i32>().unwrap();
    println!("parsed = {}", guess);

    // What changes between integer types
    let small: u8 = 255;
    // let overflow: u8 = 256;  // COMPILE ERROR: literal out of range for u8
    println!("max u8 = {}", small);
}
