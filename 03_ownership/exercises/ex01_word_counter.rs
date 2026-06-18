// Count words in a string WITHOUT taking ownership.
// Use an immutable reference to borrow the string.
// Input: "the quick brown fox"
// Expected output: 4

fn main() {
    let phrase: String = String::from("the quick brown fox");

    // Call count_words with a reference so `phrase` stays owned
    let count: usize = count_words(&phrase);

    // phrase should still be usable here
    println!("Phrase: '{}' has {} words", phrase, count);
    assert_eq!(count, 4);

    // Edge case: empty string
    let empty: String = String::new();
    let empty_count: usize = count_words(&empty);
    println!("Empty string has {} words", empty_count);
    assert_eq!(empty_count, 0);

    // Edge case: multiple spaces
    let messy: String = String::from("  spaced   out  ");
    let messy_count: usize = count_words(&messy);
    println!("Messy string has {} words", messy_count);
    assert_eq!(messy_count, 2);

    println!("✓ ex01_word_counter passed!");
}

// TODO: Implement count_words so it counts words WITHOUT taking ownership.
// Use split_whitespace() to split the string slice, then count.
// Signature: fn count_words(s: &str) -> usize
// (Using &str instead of &String makes the function more general — it accepts both.)
fn count_words(s: &str) -> usize {
    // Your code here
    todo!("implement count_words")
}
