// ex04_bonus_validate.rs — Validate input: non-empty, integer in range 1-100
// Chain ? for validation. Demonstrates progressive validation.

use std::fmt;
use std::io::{self, BufRead};
use std::num::ParseIntError;

#[derive(Debug)]
enum ValidationError {
    EmptyInput,
    ParseError(ParseIntError),
    OutOfRange { value: i32, min: i32, max: i32 },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmptyInput => write!(f, "input must not be empty"),
            ValidationError::ParseError(e) => write!(f, "parse error: {}", e),
            ValidationError::OutOfRange { value, min, max } => {
                write!(f, "{} is out of range ({}-{})", value, min, max)
            }
        }
    }
}

impl std::error::Error for ValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ValidationError::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ParseIntError> for ValidationError {
    fn from(e: ParseIntError) -> Self {
        ValidationError::ParseError(e)
    }
}

// Validate input: must be non-empty, valid i32 in range [1, 100]
fn validate(input: &str) -> Result<i32, ValidationError> {
    let trimmed = input.trim();

    // Step 1: must not be empty
    if trimmed.is_empty() {
        return Err(ValidationError::EmptyInput);
    }

    // Step 2: must parse as i32 (ParseIntError -> ValidationError via From)
    let value: i32 = trimmed.parse()?;

    // Step 3: must be in range
    if value < 1 || value > 100 {
        return Err(ValidationError::OutOfRange {
            value,
            min: 1,
            max: 100,
        });
    }

    Ok(value)
}

fn main() {
    let stdin = io::stdin();
    println!("Enter integers between 1 and 100 (one per line, Ctrl+D to quit):");

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }

        // Chain ? would work here too if main returned Result
        // But since main() returns (), we match explicitly
        match validate(&line) {
            Ok(n) => println!("  ✓ Valid: {}", n),
            Err(e) => eprintln!("  ✗ {}", e),
        }
    }
}
