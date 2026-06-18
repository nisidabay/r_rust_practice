// 03_designators.rs — $type, $expr, $ident, $block, $tt designators
//
// Designators tell the macro parser what kind of Rust syntax to match.
// Common ones:
//   $x:expr   — an expression
//   $x:ty     — a type
//   $x:ident  — an identifier (variable/function name)
//   $x:block  — a block { ... }
//   $x:stmt   — a statement
//   $x:pat    — a pattern (match arm, etc.)
//   $x:tt     — a single token tree (anything — the most flexible)
//   $x:literal — a literal (42, "hello", true)
//   $x:meta   — an attribute (like #[cfg(...)])

/// Create a function with a given name and return type.
/// Demonstrates $ident and $ty.
macro_rules! make_fn {
    ($name:ident -> $ret:ty, $body:block) => {
        fn $name() -> $ret $body
    };
}

/// Wrapper that logs entry/exit around a block.
/// Demonstrates $block.
macro_rules! log_wrap {
    ($label:expr, $block:block) => {{
        println!("[ENTER] {}", $label);
        let result = $block;
        println!("[EXIT]  {}", $label);
        result
    }};
}

/// A simple struct builder using $ident and $ty.
macro_rules! struct_builder {
    ($name:ident { $($field:ident: $ftype:ty),+ $(,)? }) => {
        struct $name {
            $(
                pub $field: $ftype,
            )+
        }
        impl $name {
            pub fn new($($field: $ftype),+) -> Self {
                Self { $($field),+ }
            }
        }
    };
}

/// Capture raw token trees with $tt — most flexible, least parsing.
macro_rules! log_tt {
    ($($tokens:tt)*) => {
        println!("[tt] {:?}", stringify!($($tokens)*));
    };
}

/// Match a literal value.
macro_rules! describe_literal {
    ($x:literal) => {
        println!("Literal: {}", $x);
    };
}

fn main() {
    println!("=== Designators in Macros ===");

    // $ident + $ty + $block — make a function
    make_fn!(greet_person -> String, {
        "Hello from a macro-generated function!".to_string()
    });
    println!("greet_person(): {}", greet_person());

    // $block — wrapping in log
    let result = log_wrap!("computation", {
        let x = 2 + 2;
        x * 10
    });
    println!("Result: {}", result);

    // $ident + $ty — struct builder
    struct_builder!(Person {
        name: String,
        age: u32,
    });

    let p = Person::new("Alice".to_string(), 30);
    println!("Person: {} is {} years old", p.name, p.age);

    // $tt — raw token trees
    log_tt!(fn foo<T: Clone>(x: T) -> T { x });
    log_tt!(if x > 0 { y } else { z });

    // $literal
    describe_literal!(42);
    describe_literal!("hello");

    println!("\nDesignator summary:");
    println!("  $x:expr    — expression");
    println!("  $x:ty      — type");
    println!("  $x:ident   — identifier or keyword");
    println!("  $x:block   — a {{ ... }} block");
    println!("  $x:tt      — any single token tree (most flexible)");
    println!("  $x:literal — literal value");
    println!("  $x:stmt    — statement");
    println!("  $x:pat     — pattern");
    println!("  $x:meta    — attribute");
}
