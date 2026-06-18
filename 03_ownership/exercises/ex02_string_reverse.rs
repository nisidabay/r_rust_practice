// Reverse a String in-place using a mutable reference (&mut String).
// Input:  "hello"
// Expected output: "olleh"

fn main() {
    let mut s: String = String::from("hello");

    // Call reverse_in_place — passes a mutable reference
    reverse_in_place(&mut s);

    println!("Reversed: {}", s);
    assert_eq!(s, "olleh");

    // Edge case: single character
    let mut single: String = String::from("a");
    reverse_in_place(&mut single);
    assert_eq!(single, "a");

    // Edge case: palindrome
    let mut pal: String = String::from("radar");
    reverse_in_place(&mut pal);
    assert_eq!(pal, "radar");

    // Edge case: empty
    let mut empty: String = String::new();
    reverse_in_place(&mut empty);
    assert_eq!(empty, "");

    println!("✓ ex02_string_reverse passed!");
}

// TODO: Implement reverse_in_place using only standard library methods.
// Hint: s is a &mut String — you can .clear() and .push_str(), or convert to
// chars, reverse, and collect. The simplest approach:
//   1. Get chars as a Vec<char>
//   2. Reverse the vec
//   3. Clear the string
//   4. Push each char back
fn reverse_in_place(s: &mut String) {
    // Your code here
    todo!("implement reverse_in_place")
}
