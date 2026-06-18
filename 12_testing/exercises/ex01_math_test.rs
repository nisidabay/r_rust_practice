// ex01_math_test.rs — Write tests for add/sub/mul/div functions covering edge cases
//
// Implement the four basic arithmetic functions below, then write tests for them.
// Edge cases to consider: overflow, negative numbers, zero, division by zero.
//
// Run: rustc --edition 2021 --test ex01_math_test.rs && ./ex01_math_test

// --- Implement these functions ---

/// Adds two signed 32-bit integers. Handles overflow by wrapping.
fn add(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

/// Subtracts `b` from `a`. Handles underflow by wrapping.
fn sub(a: i32, b: i32) -> i32 {
    a.wrapping_sub(b)
}

/// Multiplies two signed 32-bit integers. Handles overflow by wrapping.
fn mul(a: i32, b: i32) -> i32 {
    a.wrapping_mul(b)
}

/// Divides `a` by `b`. Returns `None` on division by zero.
/// Handles overflow (i32::MIN / -1) by returning None.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    // i32::MIN / -1 overflows on 2's complement
    if a == i32::MIN && b == -1 {
        return None;
    }
    Some(a / b)
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    // --- add ---

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_add_mixed_sign() {
        assert_eq!(add(5, -3), 2);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 42), 42);
        assert_eq!(add(42, 0), 42);
    }

    #[test]
    fn test_add_overflow() {
        // i32::MAX + 1 wraps to i32::MIN
        assert_eq!(add(i32::MAX, 1), i32::MIN);
    }

    #[test]
    fn test_add_underflow() {
        // i32::MIN + (-1) wraps to i32::MAX
        assert_eq!(add(i32::MIN, -1), i32::MAX);
    }

    // --- sub ---

    #[test]
    fn test_sub_positive() {
        assert_eq!(sub(10, 3), 7);
    }

    #[test]
    fn test_sub_negative() {
        assert_eq!(sub(-5, -3), -2);
    }

    #[test]
    fn test_sub_to_negative() {
        assert_eq!(sub(3, 10), -7);
    }

    #[test]
    fn test_sub_zero() {
        assert_eq!(sub(42, 0), 42);
        assert_eq!(sub(0, 42), -42);
    }

    #[test]
    fn test_sub_underflow() {
        // i32::MIN - 1 wraps to i32::MAX
        assert_eq!(sub(i32::MIN, 1), i32::MAX);
    }

    // --- mul ---

    #[test]
    fn test_mul_positive() {
        assert_eq!(mul(3, 4), 12);
    }

    #[test]
    fn test_mul_negative() {
        assert_eq!(mul(-3, 4), -12);
        assert_eq!(mul(-3, -4), 12);
    }

    #[test]
    fn test_mul_zero() {
        assert_eq!(mul(42, 0), 0);
        assert_eq!(mul(0, 42), 0);
    }

    #[test]
    fn test_mul_overflow() {
        // i32::MAX * 2 wraps
        assert_eq!(mul(i32::MAX, 2), -2);
    }

    #[test]
    fn test_mul_one() {
        assert_eq!(mul(42, 1), 42);
        assert_eq!(mul(-42, 1), -42);
    }

    #[test]
    fn test_mul_neg_one() {
        assert_eq!(mul(42, -1), -42);
        assert_eq!(mul(-42, -1), 42);
    }

    // --- div ---

    #[test]
    fn test_div_positive() {
        assert_eq!(div(10, 2), Some(5));
    }

    #[test]
    fn test_div_remainder_truncated() {
        assert_eq!(div(10, 3), Some(3)); // integer division truncates toward zero
    }

    #[test]
    fn test_div_negative() {
        assert_eq!(div(-10, 2), Some(-5));
        assert_eq!(div(10, -2), Some(-5));
        assert_eq!(div(-10, -2), Some(5));
    }

    #[test]
    fn test_div_by_zero() {
        assert_eq!(div(10, 0), None);
        assert_eq!(div(0, 0), None);
        assert_eq!(div(-5, 0), None);
    }

    #[test]
    fn test_div_zero_dividend() {
        assert_eq!(div(0, 42), Some(0));
    }

    #[test]
    fn test_div_one() {
        assert_eq!(div(42, 1), Some(42));
    }

    #[test]
    fn test_div_neg_one() {
        assert_eq!(div(42, -1), Some(-42));
    }

    #[test]
    fn test_div_overflow() {
        // i32::MIN / -1 would overflow
        assert_eq!(div(i32::MIN, -1), None);
    }

    #[test]
    fn test_div_min_by_one() {
        assert_eq!(div(i32::MIN, 1), Some(i32::MIN));
    }
}
