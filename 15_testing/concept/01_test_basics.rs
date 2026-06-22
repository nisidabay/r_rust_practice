// 01_test_basics.rs — #[test], assert!, assert_eq!, cargo test
//
// Tests can live in the same file as your code.
// Run with: rustc --test --edition 2021 01_test_basics.rs -o /tmp/test && /tmp/test
// Or: cargo test (when inside a Cargo project)

/// Adds two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Returns true if number is even
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
    assert_eq!(add(0, 0), 0);
}

#[test]
fn test_is_even() {
    assert!(is_even(2));
    assert!(is_even(0));
    assert!(!is_even(1));
    assert!(!is_even(-1));
}

#[test]
fn test_add_negative() {
    assert_eq!(add(-5, -3), -8);
}

// What you see:
//   running 3 tests
//   test test_add ... ok
//   test test_is_even ... ok
//   test test_add_negative ... ok
//
//   test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

fn main() {
    // Normal run — demonstrate the functions
    println!("add(2, 3) = {}", add(2, 3));
    println!("is_even(4)? {}", is_even(4));
    println!("\nRun with: rustc --test --edition 2021 01_test_basics.rs -o /tmp/t1 && /tmp/t1");
}
