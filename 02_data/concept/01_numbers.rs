/*
 * 01_numbers.rs — Practical Rust
 *
 * Question: How do I work with numbers in Rust?
 *
 * i32 = default integer (32-bit signed), u64 = unsigned 64-bit, f64 = default float
 * Arithmetic: + - * / %
 * Overflow: panics in debug, wraps in release (use .wrapping_*, .checked_*, .overflowing_*)
 * Casting: use `as` to convert between types
 */

fn main() {
    // i32 is the default integer type
    let a: i32 = 42;
    let b: i32 = -5;
    println!("i32: {} + {} = {}", a, b, a + b);

    // u64 — unsigned, can't be negative
    let c: u64 = 100;
    let d: u64 = 200;
    println!("u64: {} + {} = {}", c, d, c + d);

    // f64 is the default float type
    let x: f64 = 22.0;
    let y: f64 = 7.0;
    println!("f64: {} / {} = {:.6}", x, y, x / y);

    // Integer division truncates toward zero
    println!("i32 division: 10 / 3 = {}", 10 / 3);
    println!("f64 division: 10.0 / 3.0 = {:.2}", 10.0 / 3.0);

    // Remainder with %
    println!("17 % 5 = {}", 17 % 5);

    // Overflow: in debug mode this panics, in release it wraps
    // Uncomment to see panic in debug:
    // let overflow: u8 = 255 + 1; // PANICS in debug mode

    // Safe alternatives for overflow-prone code
    let val: u8 = 255;
    println!("wrapping_add: {} + 1 = {}", val, val.wrapping_add(1));
    println!("checked_add: {:?}", val.checked_add(1)); // None = overflow
    println!("saturating_add: {} + 1 = {}", val, val.saturating_add(1)); // caps at max

    // Type casting with `as`
    let pi: f64 = 3.14159;
    let truncated: i32 = pi as i32; // discards fractional part
    println!("f64 {} as i32 = {} (truncated)", pi, truncated);

    let big: i32 = 300;
    let small: u8 = big as u8; // wraps around: 300 - 256 = 44
    println!("i32 {} as u8 = {} (wraps at 256)", big, small);
}
