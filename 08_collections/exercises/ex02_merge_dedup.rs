// ex02_merge_dedup.rs — Merge two comma-separated lists, deduplicate, sort
// Read two lines from stdin, each a comma-separated list

use std::collections::BTreeSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    println!("Enter first list (comma-separated):");
    let line1 = lines.next().unwrap().unwrap();
    println!("Enter second list (comma-separated):");
    let line2 = lines.next().unwrap().unwrap();

    // BTreeSet keeps elements sorted automatically (unlike HashSet)
    let mut merged: BTreeSet<String> = BTreeSet::new();

    // Parse each line: split by comma, trim whitespace, insert
    for item in line1.split(',').chain(line2.split(',')) {
        let trimmed = item.trim().to_string();
        if !trimmed.is_empty() {
            merged.insert(trimmed);
        }
    }

    println!("\nMerged and deduplicated (sorted):");
    for item in &merged {
        println!("  {}", item);
    }
}
