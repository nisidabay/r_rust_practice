//! # integration_demo
//!
//! A library crate demonstrating unit tests (in `src/`) and integration tests (in `tests/`).
//!
//! This crate provides basic math and string utilities.
//!
//! ## Running tests
//!
//! ```bash
//! cargo test      # runs unit + integration tests
//! ```

/// Adds two numbers.
///
/// # Examples
///
/// ```
/// use integration_demo::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two numbers.
///
/// # Examples
///
/// ```
/// use integration_demo::mul;
/// assert_eq!(mul(3, 4), 12);
/// ```
pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

/// Checks if a number is even.
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Returns the maximum of two values.
pub fn max_of<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

/// Reverses a string.
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Unit tests (test private internals too) ---

    #[test]
    fn test_add_basic() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -2), -3);
    }

    #[test]
    fn test_mul_basic() {
        assert_eq!(mul(3, 4), 12);
    }

    #[test]
    fn test_mul_by_zero() {
        assert_eq!(mul(42, 0), 0);
    }

    #[test]
    fn test_is_even_true() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(-4));
    }

    #[test]
    fn test_is_even_false() {
        assert!(!is_even(1));
        assert!(!is_even(-3));
    }

    #[test]
    fn test_max_of_integers() {
        assert_eq!(max_of(3, 7), 7);
        assert_eq!(max_of(7, 3), 7);
        assert_eq!(max_of(5, 5), 5);
    }

    #[test]
    fn test_max_of_floats() {
        assert_eq!(max_of(3.14, 2.71), 3.14);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
        assert_eq!(reverse(""), "");
    }
}
