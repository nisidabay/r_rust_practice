// 05_custom_errors — Defining your own error enum, Display trait
//
// For real-world programs, define a custom error type that captures
// all the ways your application can fail. Implement Display and Debug
// (the latter via #[derive(Debug)]) so it can be used with ?.

use std::fmt;

// --- 1. Define a custom error enum ---
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

// --- 2. Implement Display (required for std::error::Error) ---
impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "cannot divide by zero"),
            MathError::NegativeSquareRoot => {
                write!(f, "cannot take square root of a negative number")
            }
            MathError::Overflow => write!(f, "result overflowed"),
        }
    }
}

// --- 3. Implement std::error::Error ---
// The Error trait requires Display + Debug. It's auto-derived once
// those are implemented, but we explicitly impl it (with no methods)
// so this type works with Box<dyn Error> and the ? operator.
impl std::error::Error for MathError {}

// --- 4. Use the custom error ---
fn safe_div(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        return Err(MathError::DivisionByZero);
    }
    Ok(a / b)
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        return Err(MathError::NegativeSquareRoot);
    }
    Ok(x.sqrt())
}

fn main() {
    // --- 5. Matching on custom errors ---
    match safe_div(10, 0) {
        Ok(v) => println!("10/0 = {v}"),
        Err(e) => println!("Error: {e}"), // uses Display
    }

    match safe_sqrt(-4.0) {
        Ok(v) => println!("sqrt(-4) = {v}"),
        Err(e) => println!("Error: {e}"),
    }

    // --- 6. Type-checking errors ---
    let result = safe_div(10, 2);
    match result {
        Ok(v) => println!("10/2 = {v}"),
        Err(MathError::DivisionByZero) => {
            println!("Tried to divide by zero (we can match specific variants)");
        }
        Err(e) => println!("Some other math error: {e}"),
    }

    // --- 7. ? propagates custom errors too ---
    fn compute_and_sqrt(a: i32, b: i32) -> Result<f64, MathError> {
        let quotient = safe_div(a, b)?.into(); // i32 -> f64
        safe_sqrt(quotient)
    }

    match compute_and_sqrt(16, 4) {
        Ok(v) => println!("sqrt(16/4) = {v}"),
        Err(e) => println!("compute_and_sqrt error: {e}"),
    }

    match compute_and_sqrt(16, 0) {
        Ok(v) => println!("sqrt(16/0) = {v}"),
        Err(e) => println!("compute_and_sqrt error: {e}"),
    }

    // --- 8. Using as Box<dyn Error> ---
    fn as_boxed() -> Result<i32, Box<dyn std::error::Error>> {
        let v = safe_div(10, 0)?; // MathError auto-converts via From
        Ok(v)
    }

    match as_boxed() {
        Ok(v) => println!("boxed: {v}"),
        Err(e) => println!("boxed error: {e}"),
    }

    println!("--- 05_custom_errors done ---");
}
