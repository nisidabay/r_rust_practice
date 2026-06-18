// Reverse a String in-place using a mutable reference (&mut String).

fn main() {
    let mut s: String = String::from("hello");
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

// Reverse string in-place by collecting chars, reversing, and rebuilding
fn reverse_in_place(s: &mut String) {
    let chars: Vec<char> = s.chars().rev().collect();
    s.clear();
    for c in chars {
        s.push(c);
    }
}
