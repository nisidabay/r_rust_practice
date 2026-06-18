// ex04_test_suite.rs (BONUS) — Build a small test harness macro
//
// Exercise: Create a `tests!` macro that defines and runs test cases.
// Each test is a named block that calls assert! and reports pass/fail status.

use std::time::Instant;

/// A test harness macro.
///
/// Syntax:
/// ```ignore
/// tests! {
///     test_name => { ... test body ... },
///     another_test => { ... },
/// }
/// ```
///
/// Each test body should evaluate to Result<(), String>.
macro_rules! tests {
    ( $name:ident => $body:expr $(,)? ) => {{
        run_single_test(stringify!($name), || -> Result<(), String> { $body });
    }};
    ( $name:ident => $body:expr, $($rest:tt)+ ) => {{
        run_single_test(stringify!($name), || -> Result<(), String> { $body });
        tests!($($rest)+);
    }};
}

/// A more expressive version that groups tests into suites.
macro_rules! test_suite {
    ($suite:ident { $($name:ident => $body:expr),+ $(,)? }) => {{
        println!("\n  ┌─ Suite: {} ───────────────────", stringify!($suite));
        let mut passed = 0u32;
        let mut failed = 0u32;
        $(
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<(), String> { $body }));
            match result {
                Ok(Ok(())) => {
                    println!("  ✓  {} — passed", stringify!($name));
                    passed += 1;
                }
                Ok(Err(msg)) => {
                    println!("  ✗  {} — FAILED: {}", stringify!($name), msg);
                    failed += 1;
                }
                Err(_) => {
                    println!("  ✗  {} — PANICKED!", stringify!($name));
                    failed += 1;
                }
            }
        )+
        println!("  └─ {} passed, {} failed\n", passed, failed);
    }};
}

/// A simple testing helper that checks equality.
/// Usage: inside a Result<(), String> function that returns Err on failure.
macro_rules! assert_eq_test {
    ($left:expr, $right:expr $(,)?) => {{
        let l = $left;
        let r = $right;
        if l != r {
            return Err(format!(
                "expected `{}` (left) == `{}` (right), but got {:?} != {:?}",
                stringify!($left), stringify!($right), l, r
            ));
        }
    }};
}

/// A testing helper that checks a boolean condition.
macro_rules! assert_test {
    ($cond:expr $(,)?) => {{
        if !$cond {
            return Err(format!("assertion failed: `{}`", stringify!($cond)));
        }
    }};
    ($cond:expr, $msg:expr $(,)?) => {{
        if !$cond {
            return Err(format!("{}: `{}`", $msg, stringify!($cond)));
        }
    }};
}

fn run_single_test(name: &str, f: fn() -> Result<(), String>) {
    let start = Instant::now();
    match f() {
        Ok(()) => println!("  ✓ {} ({:?})", name, start.elapsed()),
        Err(msg) => println!("  ✗ {} — FAILED: {} ({:?})", name, msg, start.elapsed()),
    }
}

/// A test helper function that panics if assertion fails (for use outside Result context).
macro_rules! assert_panic {
    ($cond:expr) => {{
        assert!($cond, "assert_panic failed: `{}`", stringify!($cond));
    }};
    ($left:expr, $right:expr) => {{
        assert_eq!($left, $right, "assert_panic failed: left != right");
    }};
}

fn main() {
    println!("=== Test Suite Macro ===");

    // Basic test harness
    println!("\n--- Individual tests ---");
    tests!(basic_math => {
        if 2 + 2 != 4 {
            Err("2+2 should be 4".to_string())
        } else {
            Ok(())
        }
    });
    tests!(string_length => {
        if "hello".len() != 5 {
            Err("string length mismatch".to_string())
        } else {
            Ok(())
        }
    });

    // Test suite style
    test_suite!(math_tests {
        addition => {
            if 2 + 2 != 4 { return Err("2+2 != 4".into()); }
            if 10 + 20 != 30 { return Err("10+20 != 30".into()); }
            Ok(())
        },
        subtraction => {
            if 10 - 3 != 7 { return Err("10-3 != 7".into()); }
            if !(100 > 50) { return Err("100 > 50 failed".into()); }
            Ok(())
        },
        division => {
            if 10 / 2 != 5 { return Err("10/2 != 5".into()); }
            Ok(())
        },
    });

    test_suite!(string_tests {
        concat => {
            let s = format!("{}{}", "hello", " world");
            if s != "hello world" { return Err("concat failed".into()); }
            Ok(())
        },
        contains => {
            let s = "the quick brown fox";
            if !s.contains("quick") { return Err("contains failed".into()); }
            Ok(())
        },
    });

    // Using assert_panic! (panics on failure — use in main())
    println!("\n--- Fast assertions (panic style) ---");
    assert_panic!(42 == 42);
    assert_panic!("hello".len(), 5);
    println!("  ✓ assert_panic! assertions passed");

    println!("\nBONUS features:");
    println!("- Test suites with named groups");
    println!("- Panic-catching (tests that panic won't crash the harness)");
    println!("- Pretty pass/fail output with progress");
    println!("- Manual Result-based assertions");
    println!("- assert_panic! for panic-based assertions (works in main())");
}
