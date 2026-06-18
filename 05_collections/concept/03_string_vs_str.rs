// 03_string_vs_str.rs — String vs &str, push_str, format!, to_string
//
// String owns heap data; &str is a borrowed view.
// First 5 lines: create String, push_str, format!, &str slice, to_string.

fn main() {
    // 1. String::from — owned, mutable, heap-allocated
    let mut s = String::from("hello");

    // 2. push_str — append a &str to a String
    s.push_str(", world");
    println!("{}", s); // "hello, world"

    // 3. push — append a single char
    s.push('!');
    println!("{}", s); // "hello, world!"

    // 4. format! — build a String without needing a buffer
    let name = "Alice";
    let greeting = format!("Hello, {}! You have {} new messages.", name, 3);
    println!("{greeting}");

    // 5. &str — string slice, immutable borrowed view
    let slice: &str = &s[0..5]; // "hello"
    println!("slice: {slice}");

    // 6. to_string() / to_owned() — &str → String
    let literal: &str = "I'm a &str";
    let owned: String = literal.to_string();
    let owned2: String = literal.to_owned();
    println!("{owned} | {owned2}");

    // 7. + operator — concatenation (takes ownership of left)
    let a = String::from("foo");
    let b = String::from("bar");
    let c = a + &b; // a is moved, b is borrowed
    // println!("{a}"); // ERROR: a moved
    println!("{c}");

    // 8. String vs &str in functions
    fn takes_str(s: &str) {
        println!("got: {s}");
    }
    takes_str("literal works");      // &str
    takes_str(&c[..]);               // String → &str slice
    takes_str(&c);                   // auto-deref via Deref<Target=str>

    // 9. Empty string check
    let empty = String::new();
    println!("empty? {}, len={}", empty.is_empty(), empty.len());
}
