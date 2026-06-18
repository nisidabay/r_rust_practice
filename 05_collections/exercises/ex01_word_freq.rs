// ex01_word_freq.rs — Read text, count word frequencies with HashMap
//
// Given a text string, count how many times each word appears.
// Words are case-insensitive and punctuation is stripped.

use std::collections::HashMap;

fn word_frequencies(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        // strip punctuation and lowercase
        let clean: String = word
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '\'')
            .flat_map(|c| c.to_lowercase())
            .collect();
        if !clean.is_empty() {
            *freq.entry(clean).or_insert(0) += 1;
        }
    }
    freq
}

fn main() {
    let text = "The quick brown fox jumps over the lazy dog. The dog barks, and the fox runs!";
    let freq = word_frequencies(text);

    // Print in descending order
    let mut pairs: Vec<_> = freq.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Word frequencies:");
    for (word, count) in &pairs {
        println!("  {word}: {count}");
    }

    // Expected checks
    assert_eq!(pairs.iter().find(|(w, _)| w == "the").unwrap().1, 4);
    assert_eq!(pairs.iter().find(|(w, _)| w == "fox").unwrap().1, 2);
    assert_eq!(pairs.iter().find(|(w, _)| w == "dog").unwrap().1, 2);
    assert_eq!(pairs.iter().find(|(w, _)| w == "jumps").unwrap().1, 1);

    println!("\nAll assertions passed.");
}
