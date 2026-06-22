// ex04_bonus_inverted_index.rs — Build HashMap<String, Vec<usize>>
// Maps each word to the line numbers where it appears.
// Reads stdin, tracks line numbers, builds the index.

use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut index: HashMap<String, Vec<usize>> = HashMap::new();

    for (line_num, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }

        for word in line.split_whitespace() {
            // Normalize: lowercase, strip non-alpha
            let clean: String = word
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '\'')
                .collect::<String>()
                .to_lowercase();
            if clean.is_empty() {
                continue;
            }

            // Add this line number to the word's entry
            let lines = index.entry(clean).or_insert_with(Vec::new);
            // Avoid duplicate line numbers for same word on same line
            if lines.last() != Some(&(line_num + 1)) {
                lines.push(line_num + 1);
            }
        }
    }

    // Print the inverted index sorted by word
    let mut sorted: Vec<(&String, &Vec<usize>)> = index.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));

    println!("Inverted Index (word -> line numbers):");
    for (word, lines) in &sorted {
        println!("  {} -> {:?}", word, lines);
    }
}
