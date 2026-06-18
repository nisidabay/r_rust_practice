// 06_macro_hygiene.rs — Hygiene, $crate, calling conventions
//
// Rust macros are "hygienic": names introduced in the macro expansion don't
// conflict with names in the calling scope (with some exceptions).
//
// Key concepts:
//   - Hygiene: identifiers from the macro are in a different "syntax context"
//   - $crate: absolute path to the crate where the macro is defined
//   - The `macro_use` attribute / path-based imports

/// A macro that tries to use a variable `x` from the calling scope.
/// This WILL work because `$x:expr` is passed in — the expr is resolved
/// in the caller's scope.
macro_rules! print_double {
    ($x:expr) => {
        println!("Double of {} is {}", $x, $x * 2);
    };
}

/// A macro that defines its own local variable.
/// Due to hygiene, this local `tmp` won't conflict with the caller's `tmp`.
macro_rules! swap_via_macro {
    ($a:expr, $b:expr) => {{
        let tmp = $a;
        $a = tmp;
        $b = tmp;
        // ^ This actually sets both to $a's value — just a demo
    }};
}

/// A macro that uses $crate to refer to items from its defining crate.
/// This ensures the macro works regardless of where it's imported.
macro_rules! make_crate_aware_fn {
    ($name:ident, $val:expr) => {
        fn $name() -> i32 {
            // $crate expands to the crate root — e.g., `crate`
            // Here we use it to call the macro's own helper.
            $val
        }
    };
}

/// A macro that demonstrates the "caller scope" vs "macro scope" boundary.
/// `stringify!` and `concat!` are built-in macros available everywhere.
macro_rules! show_location {
    () => {
        println!("Called from file: {}, line: {}",
                 file!(), line!());
    };
}

/// Demonstrate that `unsafe` within a macro is perfectly fine —
/// macro hygiene doesn't affect safety semantics.
macro_rules! assert_not_zero {
    ($x:expr) => {
        assert!($x != 0, "Value must not be zero!");
    };
}

fn main() {
    println!("=== Macro Hygiene ===");

    let x = 10;
    // The expression `x` is resolved in the caller's scope
    print_double!(x);

    // Hygiene: macros can't accidentally capture caller's variables
    // unless they're explicitly passed as arguments.
    let value = 42;
    print_double!(value);

    // Local variables inside macros are hygienic — they have their own context.
    let mut a = 100;
    let mut b = 200;
    {
        let tmp = "shadow"; // This is fine — the macro's `tmp` is different
    }
    // But swap_via_macro! will operate on the passed references or values
    // received by value, so it works on copies.
    println!("Before: a={}, b={}", a, b);
    swap_via_macro!(a, b); // Sets both to a's original value
    println!("After swap_via_macro: a={}, b={}", a, b);

    // file!() and line!() use the macro call site
    show_location!();

    // $crate — in a binary crate, $crate expands to just `crate`
    // In our case, this binary has no crate-level helpers, but the concept
    // is important for library authors.
    println!("In this binary, $crate == `crate` root mod");

    // Hygiene doesn't prevent calling local functions
    fn local_helper() -> &'static str {
        "local_helper() called from macro!"
    }
    // Pass as expression
    print_double!(local_helper().len() as i32);

    // Assert macro
    assert_not_zero!(5);
    // assert_not_zero!(0); // Would panic

    println!("\nHygiene rules:");
    println!("- Idents defined in macro are in their own syntax context");
    println!("- Expressions passed as args resolve in the caller's scope");
    println!("- Use $crate to refer to items from the macro's defining crate");
    println!("- file!(), line!(), column!() reflect the call site, not the macro definition");
}
