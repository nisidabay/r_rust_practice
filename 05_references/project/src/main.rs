/*
 * Project: json_extractor — extract fields from JSON lines.
 *
 * Reads lines like `{"name":"Alice","age":30}` from stdin.
 * Extracts field by name: echo '{"name":"Alice"}' | cargo run -- name
 * No serde — just find "key":"value" patterns.
 * Supports --pretty for formatted output.
 *
 * Usage:
 *   echo '{"name":"Alice","age":30}' | cargo run -- name
 *   echo '{"name":"Alice","age":30}' | cargo run -- age
 *   cat data.txt | cargo run --pretty -- name
 */

use std::env;
use std::io::{self, BufRead};

/// Extract a field value from a JSON-like string.
/// Finds "key":"value" or "key":value (numeric) patterns.
/// Returns Some(value) or None if not found.
fn extract_field<'a>(line: &'a str, field: &str) -> Option<&'a str> {
    // Look for "field":  pattern
    let search = format!("\"{}\":", field);

    // Find the key in the line
    let pos = line.find(&search)?;
    let after_colon = &line[pos + search.len()..];

    // After ":", skip whitespace
    let after = after_colon.trim_start();

    // Check if value is quoted (string) or unquoted (number/bool)
    if after.starts_with('"') {
        // String value: find closing quote
        let start = 1; // skip opening quote
        if let Some(end) = after[start..].find('"') {
            Some(&after[start..start + end])
        } else {
            None
        }
    } else {
        // Numeric value: find comma, closing brace, or end
        let end = after.find(|c: char| c == ',' || c == '}' || c == ']' || c == '\n')
            .unwrap_or(after.len());
        if end > 0 {
            Some(after[..end].trim())
        } else {
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut pretty = false;
    let mut fields: Vec<&str> = Vec::new();

    for arg in &args[1..] {
        if arg == "--pretty" {
            pretty = true;
        } else if arg.starts_with("--") {
            eprintln!("Unknown flag: {}", arg);
            std::process::exit(1);
        } else {
            fields.push(arg.as_str());
        }
    }

    if fields.is_empty() {
        eprintln!("Usage: echo '{{\"key\":\"value\"}}' | cargo run -- [--pretty] <field>");
        std::process::exit(1);
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }

        if pretty {
            println!("Input: {}", line);
        }

        for field in &fields {
            match extract_field(&line, field) {
                Some(value) => {
                    if pretty {
                        println!("  {} => {}", field, value);
                    } else {
                        println!("{}", value);
                    }
                }
                None => {
                    if pretty {
                        println!("  {} => (not found)", field);
                    }
                }
            }
        }

        if pretty {
            println!("---");
        }
    }
}
