// ex03_word_count_pipeline — Count word frequencies using a single chained iterator pipeline
//
// Given a string of text, split into words, normalize to lowercase,
// count frequencies, and print the top N words. All processing
// uses iterator chains — no explicit loops or indexing.
//
// Run with: ./ex03_word_count_pipeline [--text \"...\"] [--top N]

use std::collections::HashMap;
use std::env;

fn word_frequencies(text: &str) -> Vec<(String, usize)> {
    // Split by whitespace, clean punctuation, lowercase, count
    let mut freq: HashMap<String, usize> = HashMap::new();

    text.split_whitespace()
        .map(|word| {
            // Remove surrounding punctuation
            word.trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase()
        })
        .filter(|word| !word.is_empty()) // skip empty results
        .for_each(|word| {
            *freq.entry(word).or_insert(0) += 1;
        });

    // Sort by frequency descending, then alphabetically
    let mut result: Vec<(String, usize)> = freq.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse --text and --top arguments
    let mut text = String::new();
    let mut top_n: usize = 10;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--text" => {
                i += 1;
                if i < args.len() {
                    text = args[i].clone();
                }
            }
            "--top" => {
                i += 1;
                if i < args.len() {
                    top_n = args[i].parse().unwrap_or(10);
                }
            }
            _ => {}
        }
        i += 1;
    }

    if text.is_empty() {
        text = "The quick brown fox jumps over the lazy dog. The dog slept while the fox jumped again! \
                Rust is fast and safe. Rust is also fun. The fox is quick, the dog is lazy. \
                Hello world from Rust! This is a quick test of word counting with iterators. \
                Rust makes functional programming elegant and concise. The quick brown fox is quick indeed."
            .to_string();
    }

    let frequencies = word_frequencies(&text);

    println!("Top {top_n} word frequencies:");
    println!("{:<20} {}", "Word", "Count");
    println!("{}", "-".repeat(28));

    for (word, count) in frequencies.iter().take(top_n) {
        println!("{word:<20} {count}");
    }

    // Verify default text
    if args.len() <= 2 {
        // The word "quick" should appear multiple times
        if let Some(pos) = frequencies.iter().position(|(w, _)| w == "quick") {
            println!("\nVerification: 'quick' found at position {} with count {}", pos + 1, frequencies[pos].1);
            assert!(frequencies[pos].1 >= 3, "Expected 'quick' count >= 3");
        }
        assert!(!frequencies.is_empty(), "Expected non-empty frequencies");
        println!("Default text verification passed!");
    }
}
