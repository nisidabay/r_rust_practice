// ex03_word_pipeline.rs — Text processing pipeline
//
// Read text from stdin, apply pipeline:
//   lowercase → remove punctuation → split words → filter short words → count frequencies
//
// Use closures and iterator chains for each step.

use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    // Pipeline: read all text
    let text: String = lines
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>()
        .join("\n");

    // Helper closure: is a character punctuation?
    let is_punct = |c: char| -> bool {
        matches!(c, '.' | ',' | '!' | '?' | ';' | ':' | '"' | '\'' | '-' | '(' | ')' | '[' | ']' | '{' | '}')
    };

    // Pipeline: lowercase → remove punctuation → split → filter short words
    let words: Vec<&str> = text
        .split_whitespace()
        .collect::<Vec<&str>>();

    // Process each word: lowercase, strip punctuation, filter length >= 3
    let min_len = 3;
    let processed: Vec<String> = words.iter()
        .map(|w| w.to_lowercase())                   // lowercase
        .map(|w| w.chars().filter(|c| !is_punct(*c)).collect::<String>())  // remove punct
        .filter(|w| w.len() >= min_len)               // filter short words
        .collect();

    // Count frequencies using fold
    let counts = processed.iter().fold(HashMap::new(), |mut acc, word| {
        *acc.entry(word.clone()).or_insert(0) += 1;
        acc
    });

    // Sort by frequency (descending) and print
    let mut sorted: Vec<(&String, &i32)> = counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    println!("Word frequency (min {} chars, sorted by freq):", min_len);
    for (word, count) in sorted.iter().take(20) {
        println!("  {:15} → {}", word, count);
    }
    println!("\nTotal unique words (≥{} chars): {}", min_len, counts.len());
}
