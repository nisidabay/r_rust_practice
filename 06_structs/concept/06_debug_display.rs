/*
 * 06_debug_display.rs — Practical Rust
 *
 * Question: How do I print my structs nicely?
 *
 * #[derive(Debug)] — automatic debug formatting with {:?}
 * {:#?} — pretty-printed debug (multiline)
 * Manual Display impl — for user-facing output
 * Display requires: impl fmt::Display for MyType
 */

use std::fmt;

fn main() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 3, y: 4 };
    println!("Debug: {:?}", p);   // Point { x: 3, y: 4 }
    println!("Pretty: {:#?}", p); // multiline

    // Manual Display implementation
    struct Temperature {
        celsius: f64,
    }

    impl fmt::Display for Temperature {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:.1}°C ({:.1}°F)", self.celsius, self.celsius * 9.0 / 5.0 + 32.0)
        }
    }

    let t = Temperature { celsius: 25.0 };
    println!("{}", t); // uses Display

    // Both Debug and Display
    #[derive(Debug)]
    struct Book {
        title: String,
        author: String,
        pages: u32,
    }

    impl fmt::Display for Book {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\"{}\" by {} ({} pp)", self.title, self.author, self.pages)
        }
    }

    let book = Book {
        title: String::from("The Rust Book"),
        author: String::from("Klabnik & Nichols"),
        pages: 550,
    };

    println!("Display: {}", book);  // user-friendly
    println!("Debug:   {:?}", book); // developer-friendly
}
