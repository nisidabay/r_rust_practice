/*
 * ex02_capitalize.rs — Exercise 2
 *
 * Task: Write fn capitalize(s: &mut String) that capitalizes the first
 *       character. Test with "hello" → "Hello" and "rust" → "Rust".
 *
 * Run: ./ex02_capitalize
 * Expected: Hello, Rust, (empty), A
 */

fn capitalize(s: &mut String) {
    if let Some(c) = s.chars().next() {
        let uppercase: String = c.to_uppercase().collect();
        s.replace_range(0..1, &uppercase);
    }
}

fn main() {
    let mut test1 = String::from("hello");
    capitalize(&mut test1);
    println!("{}", test1);
    assert_eq!(test1, "Hello");

    let mut test2 = String::from("rust");
    capitalize(&mut test2);
    println!("{}", test2);
    assert_eq!(test2, "Rust");

    // Edge case: empty string
    let mut empty = String::from("");
    capitalize(&mut empty);
    println!("(empty: '{}')", empty);
    assert_eq!(empty, "");

    // Single char
    let mut single = String::from("a");
    capitalize(&mut single);
    println!("{}", single);
    assert_eq!(single, "A");

    println!("All tests passed!");
}
