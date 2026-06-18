// ex03_palindrome.rs — Check if string is palindrome (chars(), rev())
//
// A palindrome reads the same forward and backward, ignoring case,
// spaces, and punctuation.

fn is_palindrome(s: &str) -> bool {
    let clean: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();

    let reversed: String = clean.chars().rev().collect();
    clean == reversed
}

fn main() {
    let tests = [
        ("racecar", true),
        ("hello", false),
        ("A man, a plan, a canal: Panama", true),
        ("No 'x' in Nixon", true),
        ("", true),
        ("a", true),
        ("Was it a car or a cat I saw?", true),
        ("not a palindrome", false),
    ];

    for (input, expected) in &tests {
        let result = is_palindrome(input);
        let status = if result == *expected { "✓" } else { "✗" };
        println!("{status} \"{input}\" → {result} (expected {expected})");
        assert_eq!(result, *expected, "Failed on: {input}");
    }

    println!("\nAll assertions passed.");
}
