// 03_trait_basics.rs
// Defining and implementing traits.
//
// A trait defines shared behaviour: a set of method signatures that types can
// implement. Traits are Rust's version of interfaces (but more flexible).
//
// Run: rustc --edition 2021 03_trait_basics.rs && ./03_trait_basics

use std::fmt;

// ── Define a trait ───────────────────────────────────────────────────────────
// A trait named `Describe` with two methods. Types that implement Describe
// must provide bodies for both.
trait Describe {
    /// A short one-line description.
    fn describe(&self) -> String;

    /// A longer summary.
    fn summary(&self) -> String;
}

// ── Implement the trait for a struct ─────────────────────────────────────────

struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Describe for Book {
    fn describe(&self) -> String {
        format!("\"{}\" by {}", self.title, self.author)
    }

    fn summary(&self) -> String {
        format!(
            "\"{}\" by {} — {} pages",
            self.title, self.author, self.pages
        )
    }
}

// ── Implement for an enum ────────────────────────────────────────────────────

enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Describe for Temperature {
    fn describe(&self) -> String {
        match self {
            Temperature::Celsius(c) => format!("{c:.1} °C"),
            Temperature::Fahrenheit(f) => format!("{f:.1} °F"),
        }
    }

    fn summary(&self) -> String {
        match self {
            Temperature::Celsius(c) => format!("{c:.1} degrees Celsius"),
            Temperature::Fahrenheit(f) => format!("{f:.1} degrees Fahrenheit"),
        }
    }
}

// ── Implement a trait from the standard library ──────────────────────────────

struct Complex {
    real: f64,
    imag: f64,
}

// fmt::Display is a trait from std; implementing it lets us use {} in println!
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

// ── Implementing a trait for a foreign type (newtype pattern) ────────────────

/// Wrap a vector so we can implement our trait on it (orphan rule: we must
/// implement a trait on a type where either the trait or the type is local).
struct MyVec(Vec<i32>);

impl Describe for MyVec {
    fn describe(&self) -> String {
        format!("MyVec with {} elements", self.0.len())
    }

    fn summary(&self) -> String {
        let items: Vec<String> = self.0.iter().map(|x| x.to_string()).collect();
        format!("[{}]", items.join(", "))
    }
}

// ── Using traits via trait methods ───────────────────────────────────────────

fn main() {
    let book = Book {
        title: "The Rust Programming Language".into(),
        author: "Klabnik & Nichols".into(),
        pages: 560,
    };
    println!("describe: {}", book.describe());
    println!("summary:  {}", book.summary());

    let temp = Temperature::Celsius(23.5);
    println!("describe: {}", temp.describe());

    let complex = Complex {
        real: 3.0,
        imag: 4.0,
    };
    // Display — enabled by implementing fmt::Display
    println!("complex: {complex}");

    let mv = MyVec(vec![1, 2, 3, 4, 5]);
    println!("describe: {}", mv.describe());
    println!("summary:  {}", mv.summary());
}
