// Pipeline Filter — Read stdin, apply pipeline of transforms via closures
//
// Each transform is a Box<dyn Fn(String) -> String> stored in a Vec.
// Flags determine which transforms are applied and in what order.
// Demonstrates closure composition and trait objects.
//
// Usage:
//   echo "Hello World" | cargo run -- --upper --reverse
//   echo "Secret" | cargo run -- --rot13
//   echo "Hello" | cargo run -- --caesar 3

use std::env;
use std::io::{self, BufRead};

/// Apply ROT13 substitution cipher to a string
fn rot13(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
            'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
            _ => c,
        })
        .collect()
}

/// Apply Caesar cipher with given shift
fn caesar(s: &str, shift: u8) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' => (((c as u8 - b'a' + shift) % 26) + b'a') as char,
            'A'..='Z' => (((c as u8 - b'A' + shift) % 26) + b'A') as char,
            _ => c,
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Build pipeline of transforms from arguments
    let mut pipeline: Vec<Box<dyn Fn(String) -> String>> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--upper" => {
                pipeline.push(Box::new(|s| s.to_uppercase()));
            }
            "--lower" => {
                pipeline.push(Box::new(|s| s.to_lowercase()));
            }
            "--reverse" => {
                pipeline.push(Box::new(|s| s.chars().rev().collect::<String>()));
            }
            "--rot13" => {
                // rot13 doesn't capture anything, but we still need a closure
                pipeline.push(Box::new(|s| rot13(&s)));
            }
            "--caesar" => {
                i += 1;
                if i < args.len() {
                    if let Ok(shift) = args[i].parse::<u8>() {
                        // Capture `shift` by value in the move closure
                        pipeline.push(Box::new(move |s: String| caesar(&s, shift)));
                    } else {
                        eprintln!("Error: --caesar requires a number, got '{}'", args[i]);
                        return;
                    }
                } else {
                    eprintln!("Error: --caesar requires a number argument");
                    return;
                }
            }
            other => {
                eprintln!("Unknown flag: {} (try --upper, --lower, --reverse, --rot13, --caesar N)", other);
                return;
            }
        }
        i += 1;
    }

    if pipeline.is_empty() {
        eprintln!("Usage: echo \"text\" | cargo run -- [--upper] [--lower] [--reverse] [--rot13] [--caesar N]");
        return;
    }

    // Read stdin and apply pipeline to each line
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(text) => {
                let mut result = text;
                // Apply each transform in sequence — composing closures
                for transform in &pipeline {
                    result = transform(result);
                }
                println!("{}", result);
            }
            Err(e) => {
                eprintln!("Error reading stdin: {}", e);
                return;
            }
        }
    }
}
