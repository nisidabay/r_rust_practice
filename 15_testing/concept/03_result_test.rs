// 03_result_test.rs — Tests returning Result<()>
//
// Tests can return Result<()> instead of panicking.
// Use the ? operator in tests for clean error propagation.
//
// Run with: rustc --test --edition 2021 03_result_test.rs -o /tmp/t3 && /tmp/t3

use std::num::ParseIntError;

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.trim().parse()?;
    Ok(n * 2)
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test returning Result — no panic needed
    #[test]
    fn test_parse_and_double() -> Result<(), ParseIntError> {
        assert_eq!(parse_and_double("21")?, 42);
        assert_eq!(parse_and_double("0")?, 0);
        assert_eq!(parse_and_double("  -5  ")?, -10);
        Ok(())
    }

    // Can use ? inside tests!
    #[test]
    fn test_chained_result() -> Result<(), String> {
        let result = divide(10, 2)?;
        assert_eq!(result, 5);
        Ok(())
    }

    // Test that an error IS returned
    #[test]
    fn test_divide_by_zero() {
        let result = divide(10, 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "division by zero");
    }

    // Test parse error
    #[test]
    fn test_parse_error() {
        let result = parse_and_double("not_a_number");
        assert!(result.is_err());
    }

    // Both Result and non-Result tests can coexist
    #[test]
    fn test_parse_and_double_panic() {
        assert_eq!(parse_and_double("100").unwrap(), 200);
    }
}

fn main() {
    println!("parse_and_double('21') = {:?}", parse_and_double("21"));
    println!("divide(10, 3) = {:?}", divide(10, 3));
    println!("\nTests can return Result<()> for clean ? usage!");
    println!("Run: rustc --test --edition 2021 03_result_test.rs -o /tmp/t3 && /tmp/t3");
}
