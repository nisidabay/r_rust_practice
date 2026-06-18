//! Integration tests for `integration_demo`.
//!
//! Integration tests live in `tests/` and are compiled as separate crates.
//! They can only test the public API (what's `pub` in `lib.rs`).
//!
//! Run: cargo test --test integration_test
//!      cargo test          # runs all tests

use integration_demo::{add, is_even, max_of, mul, reverse};

#[test]
fn integration_test_add() {
    // Test add through its public API
    assert_eq!(add(100, 200), 300);
    assert_eq!(add(-5, 5), 0);
}

#[test]
fn integration_test_mul() {
    assert_eq!(mul(6, 7), 42);
    assert_eq!(mul(-2, 3), -6);
}

#[test]
fn integration_test_add_and_mul_compose() {
    // Test composing multiple public functions
    let result = add(mul(2, 3), mul(4, 5)); // 6 + 20
    assert_eq!(result, 26);
}

#[test]
fn integration_test_is_even() {
    assert!(is_even(0));
    assert!(is_even(100));
    assert!(!is_even(1));
    assert!(!is_even(101));
}

#[test]
fn integration_test_max_of() {
    assert_eq!(max_of(1, 100), 100);
    assert_eq!(max_of(-50, -10), -10);
    assert_eq!(max_of(42, 42), 42);
}

#[test]
fn integration_test_reverse() {
    assert_eq!(reverse("rust"), "tsur");
    assert_eq!(reverse("a"), "a");
    assert_eq!(reverse(""), "");
}

#[test]
fn integration_test_reverse_then_add() {
    // Test cross-module logic
    let s = reverse("olleh");
    // s is "hello", but integration tests can't test non-pub internals
    assert_eq!(s.len(), 5);
    assert_eq!(add(s.len() as i32, 1), 6);
}
