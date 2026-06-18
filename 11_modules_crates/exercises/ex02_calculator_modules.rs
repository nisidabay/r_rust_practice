/// Exercise 2: Split calculator functions into a `math` module
///
/// Your task:
/// 1. Create an inline module `math` with four public functions:
///    - `add(a: i32, b: i32) -> i32`
///    - `sub(a: i32, b: i32) -> i32`
///    - `mul(a: i32, b: i32) -> i32`
///    - `div(a: i32, b: i32) -> Option<i32>` (returns None on divide-by-zero)
/// 2. Call each function from main and print results.
/// 3. Add a private `validate` function that checks inputs (make it return a bool).
///
/// Run: rustc --edition 2021 ex02_calculator_modules.rs && ./ex02_calculator_modules

// TODO: Define the `math` module here

fn main() {
    // TODO: Call each math function and print results
    // Expected output format:
    // 10 + 5 = 15
    // 10 - 5 = 5
    // 10 * 5 = 50
    // 10 / 5 = Some(2)
    // 10 / 0 = None
    println!("Replace this with your solution!");
}

// Tests (do not modify)
#[test]
fn test_math_add() {
    // Only compile-test
}
