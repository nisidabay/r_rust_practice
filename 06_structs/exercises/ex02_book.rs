/*
 * ex02_book.rs — Exercise 2
 *
 * Task: Define a Book struct with title, author, pages.
 *       Implement is_long() method that returns true if pages > 300.
 *
 * Run: ./ex02_book
 * Expected: "The Rust Book" is long: true, "Thin Book" is long: false
 */

struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn new(title: &str, author: &str, pages: u32) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            pages,
        }
    }

    fn is_long(&self) -> bool {
        self.pages > 300
    }
}

fn main() {
    let book1 = Book::new("The Rust Book", "Klabnik & Nichols", 550);
    let book2 = Book::new("Thin Book", "A. Short", 50);

    println!("'{}' is long: {}", book1.title, book1.is_long());
    println!("'{}' is long: {}", book2.title, book2.is_long());

    assert!(book1.is_long());
    assert!(!book2.is_long());

    // Edge case: exactly 300 pages
    let edge = Book::new("Borderline", "Edge Case", 300);
    println!("'{}' (300 pages) is long: {}", edge.title, edge.is_long());
    assert!(!edge.is_long()); // 300 is NOT > 300

    println!("All tests passed!");
}
