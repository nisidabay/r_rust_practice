// 02_test_organization.rs — Unit tests vs integration tests, #[cfg(test)]
//
// Unit tests live in the same file as the code, guarded by #[cfg(test)] so
// they are compiled only when testing. Integration tests live in tests/ directory.
//
// Run: rustc --edition 2021 --test 02_test_organization.rs && ./02_test_organization

// --- Public API ---

/// Adds two numbers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Divides two numbers. Returns None when dividing by zero.
pub fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Returns the longer of two strings.
pub fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// --- Private helper (not pub) ---
fn double(x: i32) -> i32 {
    x * 2
}

// --- Unit tests: only compiled during `cargo test` or `rustc --test` ---
#[cfg(test)]
mod tests {
    // Bring parent module items into scope
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_div_normal() {
        assert_eq!(div(10, 2), Some(5));
    }

    #[test]
    fn test_div_by_zero() {
        assert_eq!(div(10, 0), None);
    }

    #[test]
    fn test_longer() {
        assert_eq!(longer("hi", "world"), "world");
        assert_eq!(longer("same", "same"), "same");
    }

    #[test]
    fn test_private_double() {
        // We can test private functions!
        assert_eq!(double(5), 10);
        assert_eq!(double(-3), -6);
    }
}

// Integration tests (separate file, not shown here) go in:
//   tests/integration_test.rs
// and are compiled as separate crates. They can only test the public API.
