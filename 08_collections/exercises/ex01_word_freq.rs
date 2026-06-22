// ex01_word_freq.rs — Read stdin, count word frequency, print top 5
// Uses HashMap<String, usize> to store counts, then sorts by frequency

use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut freq: HashMap<String, usize> = HashMap::new();

    // Read all lines, split into words, count each occurrence
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        for word in line.split_whitespace() {
            // Normalize: lowercase and remove non-alphabetic chars at edges
            let word: String = word
                .chars()
                .filter(|c| c.is_alphabetic() || *c == '-' || *c == '\'')
                .collect::<String>()
                .to_lowercase();
            if !word.is_empty() {
                *freq.entry(word).or_insert(0) += 1;
            }
        }
    }

    // Convert HashMap to Vec for sorting, sort by count descending
    let mut sorted: Vec<(&String, &usize)> = freq.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    // Print top 5
    println!("Top 5 words:");
    for (word, count) in sorted.iter().take(5) {
        println!("  {}: {}", word, count);
    }
}
