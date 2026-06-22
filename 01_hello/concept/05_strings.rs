/*
 * 05_strings.rs — Practical Rust
 *
 * Question: How do I work with text?
 *
 * Two string types:
 *   &str: immutable string slice ("string literal")
 *   String: heap-allocated, growable
 *
 * Convert: s.to_string(), s.to_owned(), String::from("text")
 * Use: .len(), .trim(), .to_uppercase(), .contains(), .replace()
 */

fn main() {
    // --- &str (string slice / literal) ---
    let greeting: &str = "hello";
    println!("literal: {} (len={})", greeting, greeting.len());

    // --- String (heap-allocated, growable) ---
    let mut s = String::from("hello");
    println!("String: {} (len={}, capacity={})", s, s.len(), s.capacity());

    // Append
    s.push_str(", world");
    s.push('!');
    println!("after push: {}", s);

    // --- &str vs String ---
    let literal: &str = "I'm a &str";
    let heap: String = literal.to_string();  // &str → String
    let back: &str = &heap;                   // String → &str (via borrow)
    println!("{} → String → &str again", back);

    // --- Common operations ---
    let text = String::from("  Rust Programming  ");

    // Trim whitespace
    println!("trim:    '{}'", text.trim());

    // Case conversion
    println!("upper:   {}", text.to_uppercase());
    println!("lower:   {}", text.to_lowercase());

    // Contains / replace
    println!("contains 'Rust'? {}", text.contains("Rust"));
    println!("replace: {}", text.replace("Rust", "Go"));

    // Split
    let words: Vec<&str> = text.trim().split_whitespace().collect();
    println!("words: {:?}", words);

    // Check prefix/suffix
    println!("starts with '  '? {}", text.starts_with("  "));
    println!("ends with '  '? {}", text.ends_with("  "));

    // --- Build a string efficiently ---
    let mut builder = String::new();
    builder.push_str("one");
    builder.push(',');
    builder.push_str("two");
    builder.push(',');
    builder.push_str("three");
    println!("built: {}", builder);

    // --- Format macro (like sprintf) ---
    let formatted = format!("{} + {} = {}", 3, 5, 3 + 5);
    println!("formatted: {}", formatted);
}
