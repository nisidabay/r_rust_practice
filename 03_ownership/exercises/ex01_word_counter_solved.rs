// Count words in a string WITHOUT taking ownership.
// Use an immutable reference to borrow the string.

fn main() {
    let phrase: String = String::from("the quick brown fox");
    let count: usize = count_words(&phrase);
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

// Count words by borrowing the string as &str
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}
