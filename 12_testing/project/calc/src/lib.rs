//! # calc — Library crate
//!
//! Core arithmetic operations used by the `calc` CLI binary.
//! Every function has unit tests written first (TDD approach).
//!
//! Run tests: `cargo test`

/// Adds two integers.
///
/// # Examples
///
/// ```
/// use calc::add;
/// assert_eq!(add(2, 3), 5);
/// assert_eq!(add(-1, 1), 0);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

/// Subtracts `b` from `a`.
///
/// # Examples
///
/// ```
/// use calc::sub;
/// assert_eq!(sub(10, 3), 7);
/// assert_eq!(sub(3, 10), -7);
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
    a.wrapping_sub(b)
}

/// Multiplies two integers.
///
/// # Examples
///
/// ```
/// use calc::mul;
/// assert_eq!(mul(3, 4), 12);
/// assert_eq!(mul(-2, 3), -6);
/// ```
pub fn mul(a: i32, b: i32) -> i32 {
    a.wrapping_mul(b)
}

/// Divides `a` by `b`. Returns `None` for division by zero or overflow (i32::MIN / -1).
///
/// # Examples
///
/// ```
/// use calc::div;
/// assert_eq!(div(10, 2), Some(5));
/// assert_eq!(div(10, 0), None);
/// ```
pub fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    if a == i32::MIN && b == -1 {
        return None; // overflow
    }
    Some(a / b)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- add ---
    #[test]
    fn test_add_basic() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 42), 42);
    }

    #[test]
    fn test_add_overflow() {
        assert_eq!(add(i32::MAX, 1), i32::MIN);
    }

    // --- sub ---
    #[test]
    fn test_sub_basic() {
        assert_eq!(sub(10, 3), 7);
    }

    #[test]
    fn test_sub_to_negative() {
        assert_eq!(sub(3, 10), -7);
    }

    #[test]
    fn test_sub_underflow() {
        assert_eq!(sub(i32::MIN, 1), i32::MAX);
    }

    // --- mul ---
    #[test]
    fn test_mul_basic() {
        assert_eq!(mul(3, 4), 12);
    }

    #[test]
    fn test_mul_negative() {
        assert_eq!(mul(-3, 4), -12);
    }

    #[test]
    fn test_mul_zero() {
        assert_eq!(mul(42, 0), 0);
    }

    #[test]
    fn test_mul_overflow() {
        assert_eq!(mul(i32::MAX, 2), -2);
    }

    // --- div ---
    #[test]
    fn test_div_basic() {
        assert_eq!(div(10, 2), Some(5));
    }

    #[test]
    fn test_div_by_zero() {
        assert_eq!(div(10, 0), None);
    }

    #[test]
    fn test_div_negative() {
        assert_eq!(div(-10, 2), Some(-5));
    }

    #[test]
    fn test_div_overflow() {
        assert_eq!(div(i32::MIN, -1), None);
    }

    #[test]
    fn test_div_zero_dividend() {
        assert_eq!(div(0, 42), Some(0));
    }
}
