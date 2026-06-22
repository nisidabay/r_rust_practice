// ex02_divide.rs — Read "10/3", parse numerator and denominator, handle divide by zero
// Returns Result for both parse errors and division by zero

use std::io::{self, BufRead};

#[derive(Debug)]
enum DivError {
    InvalidFormat,
    ParseError(String),
    DivideByZero,
}

impl std::fmt::Display for DivError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DivError::InvalidFormat => write!(f, "invalid format: use a/b"),
            DivError::ParseError(s) => write!(f, "parse error: '{}' is not a number", s),
            DivError::DivideByZero => write!(f, "division by zero"),
        }
    }
}

impl std::error::Error for DivError {}

fn parse_and_divide(line: &str) -> Result<f64, DivError> {
    let line = line.trim();
    // Split on '/'
    let parts: Vec<&str> = line.split('/').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(DivError::InvalidFormat);
    }

    let num: f64 = parts[0].trim().parse().map_err(|_| DivError::ParseError(parts[0].trim().to_string()))?;
    let den: f64 = parts[1].trim().parse().map_err(|_| DivError::ParseError(parts[1].trim().to_string()))?;

    if den == 0.0 {
        return Err(DivError::DivideByZero);
    }

    Ok(num / den)
}

fn main() {
    let stdin = io::stdin();
    println!("Enter expressions like '10/3' (Ctrl+D to quit):");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }

        match parse_and_divide(&line) {
            Ok(result) => println!("= {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
