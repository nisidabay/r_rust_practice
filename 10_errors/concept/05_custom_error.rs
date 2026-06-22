fn main() {
    // Custom error types — define your own enum implementing std::error::Error
    // This gives you structured error information, not just strings.

    use std::fmt;

    // --- Define a custom error enum ---
    // Derive Debug (required by Error trait), implement Display manually
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeSquareRoot,
        Overflow,
    }

    // Display is required by the Error trait — gives user-friendly messages
    impl fmt::Display for MathError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                MathError::DivisionByZero => write!(f, "cannot divide by zero"),
                MathError::NegativeSquareRoot => write!(f, "cannot take square root of negative number"),
                MathError::Overflow => write!(f, "numeric overflow"),
            }
        }
    }

    // Implement the Error trait (no additional methods required with Display)
    impl std::error::Error for MathError {}

    // --- Use the custom error type ---
    fn safe_div(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }

    fn safe_sqrt(x: f64) -> Result<f64, MathError> {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    // Usage — match on specific error variants
    let results = vec![safe_div(10.0, 2.0), safe_div(5.0, 0.0), safe_sqrt(-1.0)];
    for r in results {
        match r {
            Ok(val) => println!("Result: {}", val),
            Err(e) => println!("Error: {}", e),  // Display impl makes this clean
        }
    }

    // Works with ? operator too
    fn compute(a: f64, b: f64) -> Result<f64, MathError> {
        let quotient = safe_div(a, b)?;
        safe_sqrt(quotient)
    }

    println!("\ncompute(25.0, 5.0) = {:?}", compute(25.0, 5.0));
    println!("compute(25.0, 0.0) = {:?}", compute(25.0, 0.0));
}
