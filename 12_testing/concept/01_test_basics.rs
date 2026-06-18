// 01_test_basics.rs — #[test], assert!, assert_eq!, assert_ne!, should_panic
//
// The #[test] attribute turns a plain function into a test.
// cargo will discover and run all #[test] functions automatically.
// Use rustc --test to compile tests outside Cargo.
//
// Run: rustc --edition 2021 --test 01_test_basics.rs && ./01_test_basics

/// Basic #[test] function — Rust finds it by the attribute.
#[test]
fn test_addition_simple() {
    assert!(2 + 2 == 4); // assert! takes a bool
}

/// assert_eq! compares two values for equality (uses PartialEq + Debug).
#[test]
fn test_assert_eq_example() {
    assert_eq!(2 + 2, 4);
}

/// assert_ne! asserts two values are NOT equal.
#[test]
fn test_assert_ne_example() {
    assert_ne!(2 + 2, 5);
}

/// Custom failure messages — the extra arguments format a message on failure.
#[test]
fn test_with_message() {
    let result = 2 + 2;
    assert!(
        result == 4,
        "Expected 4 but got {result}"
    );
    assert_eq!(result, 4, "2 + 2 should be 4, got {result}");
}

/// #[should_panic] — the test passes only if the code panics.
#[test]
#[should_panic]
fn test_panics() {
    panic!("This panic is expected!");
}

/// #[should_panic(expected = "...")] — also checks the panic message.
#[test]
#[should_panic(expected = "divide by zero")]
fn test_panic_expected_message() {
    // Dynamic value prevents compile-time detection of the division by zero
    let n = std::env::args().len() as i32; // runtime value, not known at compile time
    let _ = 1 / (n - n); // always divides by zero
}

/// assert! on more complex conditions.
#[test]
fn test_assert_boolean_expression() {
    let x = 42;
    assert!(x > 0 && x < 100, "x should be in range (0, 100)");
}

// ---
// To run these with rustc directly:
//   rustc --edition 2021 --test 01_test_basics.rs
//   ./01_test_basics
//
// With Cargo (once in a crate):
//   cargo test
