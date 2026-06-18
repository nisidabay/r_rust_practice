// 04_result_in_tests.rs — Returning Result<()> from tests for ? operator
//
// Tests can return Result<(), E> instead of panicking. This lets you use ?
// for clean error propagation and get rich Debug error messages on failure.
//
// Run: rustc --edition 2021 --test 04_result_in_tests.rs && ./04_result_in_tests

use std::num::ParseIntError;
use std::str::FromStr;

// --- Functions to test ---

/// Parses a string as a u32. Can fail.
fn parse_u32(s: &str) -> Result<u32, ParseIntError> {
    u32::from_str(s)
}

/// Reads a number from a string and doubles it.
fn double_str(s: &str) -> Result<u32, ParseIntError> {
    let n = parse_u32(s)?;
    Ok(n * 2)
}

/// Divides two parsed numbers.
fn div_strs(a: &str, b: &str) -> Result<u32, String> {
    let x = parse_u32(a).map_err(|e| format!("first arg: {e}"))?;
    let y = parse_u32(b).map_err(|e| format!("second arg: {e}"))?;
    if y == 0 {
        return Err("division by zero".into());
    }
    Ok(x / y)
}

// --- Tests returning Result<()> ---
// The test passes if Ok(()) is returned; fails if Err(...) is returned.

#[test]
fn test_parse_valid() -> Result<(), ParseIntError> {
    let n = parse_u32("42")?;
    assert_eq!(n, 42);
    Ok(())
}

#[test]
fn test_double_str_valid() -> Result<(), ParseIntError> {
    let n = double_str("21")?;
    assert_eq!(n, 42);
    Ok(())
}

#[test]
fn test_div_strs_valid() -> Result<(), String> {
    let n = div_strs("10", "2")?;
    assert_eq!(n, 5);
    Ok(())
}

#[test]
fn test_div_strs_by_zero() -> Result<(), String> {
    let result = div_strs("10", "0");
    assert!(result.is_err(), "should error on divide by zero");
    // We don't use ? here because we expect an error
    Ok(())
}

#[test]
fn test_parse_invalid() {
    // This test uses traditional panic-on-error
    let result = parse_u32("not_a_number");
    assert!(result.is_err());
}

// --- Bonus: combining Result with ? and assert_eq! ---
#[test]
fn test_chained_operations() -> Result<(), Box<dyn std::error::Error>> {
    // Box<dyn Error> lets us use ? with any error type
    let a: u32 = "10".parse()?;
    let b: u32 = "5".parse()?;
    assert_eq!(a + b, 15);
    assert_eq!(a - b, 5);
    assert_eq!(a * b, 50);
    assert_eq!(a / b, 2);
    Ok(())
}
