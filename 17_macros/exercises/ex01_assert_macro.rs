// ex01_assert_macro.rs — Build a custom assert! equivalent with better messages
//
// Exercise: Create a `custom_assert!` macro that provides more detailed failure
// messages than the standard assert!, showing both the expression as a string
// and the actual values.

// Demonstration of stringify! inside assert
macro_rules! custom_assert {
    // Assert with message override
    ($cond:expr, $msg:expr) => {
        if !$cond {
            panic!("ASSERT FAILED: {}\n  Condition: {}\n  File: {}:{}",
                   $msg, stringify!($cond), file!(), line!());
        }
    };
    // Assert with default message
    ($cond:expr) => {
        if !$cond {
            panic!("ASSERT FAILED: `{}` was false\n  File: {}:{}",
                   stringify!($cond), file!(), line!());
        }
    };
}

/// A more feature-rich version with value display.
/// Shows both sides of comparison assertions.
macro_rules! assert_eq_custom {
    ($left:expr, $right:expr) => {{
        let l = $left;
        let r = $right;
        if l != r {
            panic!(
                "ASSERT_EQ FAILED\n  Left  ({}): {:?}\n  Right ({}): {:?}\n  File: {}:{}",
                stringify!($left), l,
                stringify!($right), r,
                file!(), line!()
            );
        }
    }};
    ($left:expr, $right:expr, $msg:expr) => {{
        let l = $left;
        let r = $right;
        if l != r {
            panic!(
                "ASSERT_EQ FAILED: {}\n  Left  ({}): {:?}\n  Right ({}): {:?}\n  File: {}:{}",
                $msg,
                stringify!($left), l,
                stringify!($right), r,
                file!(), line!()
            );
        }
    }};
}

/// An assert_ne! equivalent.
macro_rules! assert_ne_custom {
    ($left:expr, $right:expr) => {{
        let l = $left;
        let r = $right;
        if l == r {
            panic!(
                "ASSERT_NE FAILED — values are equal\n  Left  ({}): {:?}\n  Right ({}): {:?}\n  File: {}:{}",
                stringify!($left), l,
                stringify!($right), r,
                file!(), line!()
            );
        }
    }};
}

fn main() {
    println!("=== Custom Assert Macro ===");

    // These should pass
    custom_assert!(true);
    custom_assert!(2 + 2 == 4, "Basic arithmetic works");
    assert_eq_custom!(10, 10);
    assert_eq_custom!(10, 10, "ten equals ten");
    assert_ne_custom!(10, 20);

    println!("All assertions passed!");

    // Uncomment to see failure messages:
    // custom_assert!(false, "This should fail");
    // custom_assert!(2 + 2 == 5);
    // assert_eq_custom!(10, 20);
    // assert_eq_custom!(10, 20, "expected ten to be twenty");
    // assert_ne_custom!(10, 10);

    println!("\nKey features:");
    println!("- stringify!($cond) shows the expression text");
    println!("- file!() and line!() show where the failure occurred");
    println!("- Custom messages supported");
    println!("- Assert_eq shows both sides with proper labeling");
}
