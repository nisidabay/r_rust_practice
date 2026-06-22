// 04_doc_test.rs — /// ```rust blocks. cargo test runs doc tests.
//
// Documentation tests let you embed runnable examples in your docs.
// They're tested with: cargo test
//
// Run: rustc --test --edition 2021 04_doc_test.rs -o /tmp/t4 && /tmp/t4
// Note: doc tests are only picked up by `cargo test`, not `rustc --test`.
// This file shows OFFLINE equivalent — but doc tests need cargo.

/// Adds two numbers.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// ```
/// assert_eq!(add(-1, 1), 0);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Divides two numbers. Returns None if dividing by zero.
///
/// # Examples
///
/// ```
/// let result = safe_divide(10, 2);
/// assert_eq!(result, Some(5));
/// ```
///
/// ```
/// let result = safe_divide(10, 0);
/// assert_eq!(result, None);
/// ```
pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

// Regular tests (same as before)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Some(5));
        assert_eq!(safe_divide(10, 0), None);
    }
}

fn main() {
    println!("add(2, 3) = {}", add(2, 3));
    println!("safe_divide(10, 2) = {:?}", safe_divide(10, 2));
    println!("\nDoc tests run with: cargo test");
    println!("They ensure your code examples in docs are always correct.");
}
