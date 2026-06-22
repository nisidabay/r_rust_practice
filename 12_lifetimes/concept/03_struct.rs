// 03_struct.rs — Struct with references: must annotate lifetimes
//
// When a struct holds references, Rust needs to know how long they're valid.
// Every reference in a struct gets a lifetime parameter on the struct itself.

// Without <'a>: "missing lifetime specifier"
// struct Book { title: &str }  // ERROR
//
// 'a says: "this Book instance can't outlive the string slice it borrows"
struct Book<'a> {
    title: &'a str,
    pages: u32,
}

// The impl block also needs the lifetime parameter
impl<'a> Book<'a> {
    // Constructor — returns a Book that borrows from given title
    fn new(title: &'a str, pages: u32) -> Self {
        Book { title, pages }
    }

    // Getter — output lifetime linked to self's lifetime (elision rule 3)
    fn title(&self) -> &str {
        self.title
    }

    // Method that takes another reference with same lifetime
    fn compare_title(&self, other: &'a str) -> bool {
        self.title.len() > other.len()
    }
}

fn main() {
    // title is a String, Book borrows from it
    let title = String::from("The Rust Programming Language");
    let book = Book::new(&title, 552);

    println!("Book: '{}' ({} pages)", book.title(), book.pages);

    // Compare with another borrowed string
    let other = "Rust";
    println!("Title longer than '{}'? {}", other, book.compare_title(other));

    // The compiler ensures the Book doesn't outlive its title
    // std::mem::drop(title);
    // println!("{}", book.title());  // WOULD NOT COMPILE — title is gone

    println!("Struct lifetimes: Book borrows '{}', and can't outlive it.", title);
}
