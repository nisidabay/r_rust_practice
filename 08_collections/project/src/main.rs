// Project: frequency counter
// Counts character, word, and line frequency from stdin.
// Prints histogram-style output with --sort (alpha|freq) and --min-count N.
//
// Usage:
//   echo "hello world hello" | cargo run --
//   echo "foo foo bar" | cargo run -- --sort freq --min-count 2

use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

struct Config {
    sort: SortOrder,
    min_count: usize,
}

enum SortOrder {
    Alpha,  // sort alphabetically
    Freq,   // sort by frequency (descending)
}

fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let mut sort = SortOrder::Alpha;
    let mut min_count = 1usize;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--sort" => {
                i += 1;
                if i < args.len() {
                    sort = match args[i].as_str() {
                        "freq" => SortOrder::Freq,
                        _ => SortOrder::Alpha,
                    };
                }
            }
            "--min-count" => {
                i += 1;
                if i < args.len() {
                    min_count = args[i].parse().unwrap_or(1);
                }
            }
            _ => {}
        }
        i += 1;
    }

    Config { sort, min_count }
}

fn print_histogram(title: &str, counts: &HashMap<String, usize>, config: &Config) {
    // Filter by min-count
    let mut items: Vec<(&String, &usize)> = counts.iter().filter(|(_, &c)| c >= config.min_count).collect();

    // Sort
    match config.sort {
        SortOrder::Alpha => items.sort_by(|a, b| a.0.cmp(b.0)),
        SortOrder::Freq => items.sort_by(|a, b| b.1.cmp(a.1)),
    }

    if items.is_empty() {
        return;
    }

    // Find max count for histogram scaling
    let max_count = *items.iter().map(|(_, c)| *c).max().unwrap_or(&1);
    let bar_width = 40usize;

    println!("\n{} (min-count={}):", title, config.min_count);
    for (key, count) in items {
        let bar_len = if max_count > 0 {
            (*count * bar_width) / max_count
        } else {
            0
        };
        let bar = "#".repeat(bar_len);
        println!("  {:20} {:5} {}", key, count, bar);
    }
}

fn main() {
    let config = parse_args();
    let stdin = io::stdin();

    let mut char_counts: HashMap<String, usize> = HashMap::new();
    let mut word_counts: HashMap<String, usize> = HashMap::new();
    let mut line_count = 0usize;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        line_count += 1;

        // Count characters (non-whitespace)
        for c in line.chars().filter(|c| !c.is_whitespace()) {
            *char_counts.entry(c.to_string()).or_insert(0) += 1;
        }

        // Count words
        for word in line.split_whitespace() {
            let clean: String = word
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '\'')
                .collect::<String>()
                .to_lowercase();
            if !clean.is_empty() {
                *word_counts.entry(clean).or_insert(0) += 1;
            }
        }
    }

    println!("Lines: {}", line_count);
    print_histogram("Characters", &char_counts, &config);
    print_histogram("Words", &word_counts, &config);
}
