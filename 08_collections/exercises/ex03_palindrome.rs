// ex03_palindrome.rs — Read lines, check palindrome (ignoring case + spaces)
// A palindrome reads the same forwards and backwards.
// Uses Vec<char> or String methods to normalize and compare.

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    println!("Enter strings (one per line, Ctrl+D to quit):");

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Normalize: lowercase, keep only alphanumeric chars
        let clean: String = trimmed
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        // Check palindrome using Vec<char> and two-pointer comparison
        let chars: Vec<char> = clean.chars().collect();
        let mut is_pal = true;
        for i in 0..chars.len() / 2 {
            if chars[i] != chars[chars.len() - 1 - i] {
                is_pal = false;
                break;
            }
        }

        if is_pal {
            println!("  ✓ palindrome: {}", trimmed);
        } else {
            println!("  ✗ not palindrome: {}", trimmed);
        }
    }
}
