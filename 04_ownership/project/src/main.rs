/*
 * Project: string_sanitizer — read stdin, apply filters, print result.
 *
 * Filters (applied in order):
 *   --lower        Convert to lowercase
 *   --upper        Convert to uppercase
 *   --trim         Trim leading/trailing whitespace
 *   --no-spaces    Collapse all whitespace (replace runs with single space)
 *   --alpha-only   Keep only alphabetic characters (a-zA-Z) and spaces
 *
 * Usage:
 *   echo "Hello  WORLD!!" | cargo run -- --lower --no-spaces
 *   cat file.txt | cargo run -- --alpha-only --trim
 */

use std::env;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let flags: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();

    // Read all input from stdin
    let mut input = String::new();
    let stdin = std::io::stdin();
    let _ = std::io::Read::read_to_string(&mut stdin.lock(), &mut input);

    let mut result = input;

    // Apply filters in order
    for flag in &flags {
        match *flag {
            "--lower" => {
                result = result.to_lowercase();
            }
            "--upper" => {
                result = result.to_uppercase();
            }
            "--trim" => {
                result = result.trim().to_string();
            }
            "--no-spaces" => {
                // Collapse runs of whitespace into single spaces
                let mut out = String::with_capacity(result.len());
                let mut in_whitespace = false;
                for c in result.chars() {
                    if c.is_whitespace() {
                        if !in_whitespace {
                            out.push(' ');
                            in_whitespace = true;
                        }
                    } else {
                        out.push(c);
                        in_whitespace = false;
                    }
                }
                result = out;
            }
            "--alpha-only" => {
                // Keep only a-zA-Z and spaces
                let mut out = String::with_capacity(result.len());
                for c in result.chars() {
                    if c.is_ascii_alphabetic() || c == ' ' || c == '\n' {
                        out.push(c);
                    }
                }
                result = out;
            }
            other => {
                eprintln!("Unknown flag: {}", other);
                std::process::exit(1);
            }
        }
    }

    print!("{}", result);
}
