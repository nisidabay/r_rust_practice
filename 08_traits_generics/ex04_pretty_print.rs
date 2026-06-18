// ex04_pretty_print.rs
// BONUS: PrettyPrint trait with default impl + custom for your types.
//
// Design a trait that provides pretty-printed output for any type. The trait
// should have a default implementation that falls back to Debug formatting,
// but types can override it for custom display.
//
// This version uses a supertrait (Debug) so the default works, and individual
// types can still override pretty_print() with custom formatting.
//
// Key learning: Rust does NOT allow overriding default methods from blanket
// impls. If you write `impl<T: Debug> PrettyPrint for T {}`, no individual
// type can override the pretty_print method. The correct pattern is to
// implement the trait explicitly for each type (or use conditional impls
// for groups of types).
//
// Run: rustc --edition 2021 ex04_pretty_print.rs && ./ex04_pretty_print

use std::fmt::Debug;

// ── The trait ─────────────────────────────────────────────────────────────────

/// A trait for types that can be pretty-printed.
///
/// Debug is a supertrait: only types that implement Debug can implement
/// PrettyPrint. The default implementation uses Debug formatting.
trait PrettyPrint: Debug {
    /// Output a nicely formatted representation of self.
    /// Default: uses Debug format.
    fn pretty_print(&self) -> String {
        format!("{:?}", self)
    }
}

// ── Implementations for standard types ────────────────────────────────────────

impl PrettyPrint for i32 {}
impl PrettyPrint for f64 {}
impl PrettyPrint for String {}
impl PrettyPrint for &str {}
impl<T: Debug> PrettyPrint for Vec<T> {}

// ── Custom types ──────────────────────────────────────────────────────────────

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

impl PrettyPrint for Person {
    fn pretty_print(&self) -> String {
        format!(
            "Person {{\n  name:  {}\n  age:   {}\n  email: {}\n}}",
            self.name, self.age, self.email
        )
    }
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: u16,
}

impl PrettyPrint for Book {
    fn pretty_print(&self) -> String {
        format!(
            "📖 \"{}\" by {} ({})",
            self.title, self.author, self.year
        )
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl PrettyPrint for Point {
    fn pretty_print(&self) -> String {
        format!("📍 ({:.1}, {:.1})", self.x, self.y)
    }
}

// ── Generic function ──────────────────────────────────────────────────────────

/// Pretty-prints each element in a slice.
fn pretty_print_all<T: PrettyPrint>(items: &[T]) {
    for (i, item) in items.iter().enumerate() {
        println!("  [{i}]: {}", item.pretty_print());
    }
}

// ── Vec wrapper with custom pretty print ─────────────────────────────────────

#[derive(Debug)]
struct Table<T: Debug>(Vec<T>);

impl<T: Debug> PrettyPrint for Table<T> {
    fn pretty_print(&self) -> String {
        let mut out = String::from("Table {\n");
        for (i, item) in self.0.iter().enumerate() {
            out.push_str(&format!("  [{i}]: {:?}\n", item));
        }
        out.push('}');
        out
    }
}

fn main() {
    // Standard types get the default (Debug) implementation.
    println!("Standard types (default Debug impl):");
    println!("  42:        {}", 42i32.pretty_print());
    println!("  3.14:      {}", 3.14f64.pretty_print());
    println!("  \"hello\":  {}", "hello".pretty_print());
    println!("  vec:       {}", vec![1, 2, 3].pretty_print());

    // Custom types with overridden PrettyPrint
    println!("\nCustom Person (overridden):");
    let alice = Person {
        name: "Alice".into(),
        age: 30,
        email: "alice@example.com".into(),
    };
    println!("{}", alice.pretty_print());

    println!("\nCustom Book (overridden):");
    let book = Book {
        title: "Rust in Action".into(),
        author: "Tim McNamara".into(),
        year: 2021,
    };
    println!("{}", book.pretty_print());

    println!("\nCustom Point (overridden):");
    let pt = Point { x: 3.5, y: 7.2 };
    println!("{}", pt.pretty_print());

    // Generic slice pretty-printing
    println!("\nPretty-print all in a slice:");
    let people = vec![
        Person {
            name: "Bob".into(),
            age: 25,
            email: "bob@example.com".into(),
        },
        Person {
            name: "Carol".into(),
            age: 28,
            email: "carol@example.com".into(),
        },
    ];
    pretty_print_all(&people);

    // Table with custom formatting
    println!("\nTable:");
    let table = Table(vec![10, 20, 30, 40, 50]);
    println!("{}", table.pretty_print());

    // ── Assertions ───────────────────────────────────────────────────────────
    assert_eq!(42i32.pretty_print(), "42");
    assert_eq!(3.14f64.pretty_print(), "3.14");
    assert!(alice.pretty_print().contains("Alice"));
    assert!(book.pretty_print().contains("📖"));
    assert!(Point { x: 1.0, y: 2.0 }.pretty_print().contains("📍"));

    println!("\nAll assertions passed! ✓");
}
