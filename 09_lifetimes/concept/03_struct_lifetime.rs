// 03_struct_lifetime.rs
// Structs holding references: struct Foo<'a> — lifetime parameters on structs.
//
// When a struct holds a reference, Rust needs to know how long that reference
// is valid. You add a lifetime parameter to the struct definition, then use
// it on every reference field.
//
// A struct with a lifetime parameter means: "This struct cannot outlive the
// data it references."
//
// The lifetime parameter MUST appear in the struct name at every use site:
//   let parser = Parser { input: &data };
// won't compile without: Parser<'_> or explicit Parser<'a>
//
// Run: rustc --edition 2021 03_struct_lifetime.rs && ./03_struct_lifetime

/// A simple struct that borrows a string slice.
/// 'a means "the struct lives at most as long as the data in `text`"
#[derive(Debug)]
struct Excerpt<'a> {
    text: &'a str,
    source: &'a str,
}

impl<'a> Excerpt<'a> {
    /// Returns the first line of the excerpt.
    /// The return type &str gets the same lifetime 'a via elision rule 3 (&self).
    fn first_line(&self) -> &str {
        match self.text.find('\n') {
            Some(pos) => &self.text[..pos],
            None => self.text,
        }
    }

    /// Returns both the excerpt and its source.
    fn parts(&self) -> (&str, &str) {
        (self.text, self.source)
    }
}

/// A parser that borrows its input string.
/// This is the most common use case: parse data without allocating copies.
#[derive(Debug)]
struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    /// Return the next word (sequence of non-whitespace chars).
    /// Returns a reference INTO the original input — needs 'a on output.
    fn next_word(&mut self) -> Option<&'a str> {
        // Skip whitespace
        while self.pos < self.input.len() && self.input.as_bytes()[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        if self.pos >= self.input.len() {
            return None;
        }

        let start = self.pos;
        while self.pos < self.input.len() && !self.input.as_bytes()[self.pos].is_ascii_whitespace()
        {
            self.pos += 1;
        }
        Some(&self.input[start..self.pos])
    }

    /// Check if we've consumed all input.
    fn is_done(&self) -> bool {
        self.pos >= self.input.len()
    }
}

/// A generic struct that borrows data of any type.
/// Works exactly the same way — lifetime on the struct, reference field.
#[derive(Debug)]
struct Borrowed<'a, T: ?Sized> {
    value: &'a T,
}

fn main() {
    // --- Excerpt ---
    let article = String::from("Rust is amazing\nOwnership without GC\nFast and safe");
    let excerpt = Excerpt {
        text: &article,
        source: "Rust Book",
    };
    println!("Excerpt struct: {excerpt:?}");
    println!("First line: '{}'", excerpt.first_line());
    let (text, source) = excerpt.parts();
    println!("Parts: text='{text}', source='{source}'");

    // Excerpt is dropped here — article is still alive, so excerpt is fine ✓
    // If excerpt outlived article, the compiler would refuse

    // --- Parser ---
    let data = "hello world from Rust";
    let mut parser = Parser::new(data);
    println!("\nParsing: '{data}'");
    while let Some(word) = parser.next_word() {
        println!("  word: '{word}' (type: &str)");
    }
    println!("Parser done: {}", parser.is_done());
    // parser borrows `data`, so data can't be moved while parser lives ✓

    // --- Borrowed with different types ---
    let num = 42i32;
    let borrowed_int: Borrowed<'_, i32> = Borrowed { value: &num };
    println!("\nBorrowed i32: {}", borrowed_int.value);

    let text = String::from("generic string");
    let borrowed_str: Borrowed<'_, str> = Borrowed {
        value: &text[..],
    };
    println!("Borrowed str: {}", borrowed_str.value);

    // --- Lifetime constraint in action ---
    // The following BLOCKING code demonstrates WHY lifetimes prevent bugs.
    // Uncomment to see the compile error:

    // let outer_ref;
    // {
    //     let local = String::from("temporary data");
    //     // Creating an excerpt that references `local` — local lives shorter
    //     let excerpt = Excerpt { text: &local, source: "temp" };
    //     outer_ref = &excerpt;
    // }
    // // outer_ref now points to freed data — COMPILE ERROR prevented!
    // println!("{}", outer_ref.first_line());

    println!("\n✓ All struct lifetimes work correctly!");
}
