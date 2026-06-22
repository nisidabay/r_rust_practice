// ex03_file_stats.rs — Read files from args, print lines/words/chars for each
//
// Like `wc` but simpler: count lines, words, and characters for each file.
// Print a total row at the bottom.
//
// Usage: ex03_file_stats <file1> [file2 ...]
//   ex03_file_stats Cargo.toml
//   ex03_file_stats file1.txt file2.txt

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct FileStats {
    lines: usize,
    words: usize,
    chars: usize,
}

fn count_stats(filename: &str) -> Result<FileStats, String> {
    let file = File::open(filename).map_err(|e| format!("Error opening '{}': {}", filename, e))?;
    let reader = BufReader::new(file);

    let mut stats = FileStats {
        lines: 0,
        words: 0,
        chars: 0,
    };

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Error reading '{}': {}", filename, e))?;
        stats.lines += 1;
        stats.words += line.split_whitespace().count();
        stats.chars += line.chars().count();
    }

    Ok(stats)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file1> [file2 ...]", args[0]);
        return;
    }

    let files: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();
    let mut grand_total = FileStats {
        lines: 0,
        words: 0,
        chars: 0,
    };

    // Print header
    println!("{:>8} {:>8} {:>8}  {}", "Lines", "Words", "Chars", "File");
    println!("{}", "-".repeat(50));

    for filename in &files {
        match count_stats(filename) {
            Ok(stats) => {
                println!(
                    "{:>8} {:>8} {:>8}  {}",
                    stats.lines, stats.words, stats.chars, filename
                );
                grand_total.lines += stats.lines;
                grand_total.words += stats.words;
                grand_total.chars += stats.chars;
            }
            Err(e) => {
                eprintln!("  {:>8} {:>8} {:>8}  {} (error)", "-", "-", "-", filename);
                eprintln!("  {}", e);
            }
        }
    }

    // Print total row only if multiple files
    if files.len() > 1 {
        println!("{}", "-".repeat(50));
        println!(
            "{:>8} {:>8} {:>8}  {}",
            grand_total.lines, grand_total.words, grand_total.chars, "(total)"
        );
    }
}
