// 05_doc_tests.rs — /// ``` code blocks ```, cargo test --doc
//
// Doc tests are code examples in doc comments that `cargo test` runs.
// They serve dual purpose: documentation + verification.
//
// Run: rustc --edition 2021 --test 05_doc_tests.rs && ./05_doc_tests
// But doc tests are best run with: cargo test --doc
// (this file also has regular #[test] tests for completeness)

// --- Doc tests on public items ---

/// Adds two numbers and returns the result.
///
/// # Examples
///
/// ```
/// // This code block is a doc test — it's compiled and run by `cargo test`
/// use doc_tests::add;
///
/// assert_eq!(add(2, 3), 5);
/// assert_eq!(add(-1, 1), 0);
/// ```
///
/// # Edge cases
///
/// ```
/// assert_eq!(doc_tests::add(0, 0), 0);
/// assert_eq!(doc_tests::add(-5, -5), -10);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Greets a person by name.
///
/// # Examples
///
/// ```
/// assert_eq!(doc_tests::greet("Alice"), "Hello, Alice!");
/// assert_eq!(doc_tests::greet("Bob"), "Hello, Bob!");
/// ```
///
/// An empty name still works:
///
/// ```
/// assert_eq!(doc_tests::greet(""), "Hello, !");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Calculates the length of a string slice.
///
/// # Examples
///
/// ```
/// assert_eq!(doc_tests::str_len("hello"), 5);
/// assert_eq!(doc_tests::str_len(""), 0);
/// assert_eq!(doc_tests::str_len("🦀"), 4); // multi-byte char
/// ```
pub fn str_len(s: &str) -> usize {
    s.len()
}

// --- Doc tests can also be written as "compile-fail" tests ---

/// This function panics if given zero.
///
/// # Panics
///
/// Panics if `divisor` is 0.
///
/// # Examples
///
/// ```
/// assert_eq!(doc_tests::safe_div(10, 2), 5);
/// ```
///
/// ```should_panic
/// // This doc test is expected to panic
/// doc_tests::safe_div(10, 0);
/// ```
pub fn safe_div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}

// --- Doc tests ignore code blocks with "ignore" or "no_run" ---

/// ```
/// // This block is ignored — not compiled or run
/// # // The # hides this line from docs but still runs it in doc tests
/// # assert_eq!(1 + 1, 2);
/// ```
pub fn ignored_example() {}

// --- Regular unit tests (for completeness with rustc --test) ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_direct() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_greet_direct() {
        assert_eq!(greet("World"), "Hello, World!");
    }

    #[test]
    fn test_str_len_direct() {
        assert_eq!(str_len("abc"), 3);
    }
}
