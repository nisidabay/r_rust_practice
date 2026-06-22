// ex01_head.rs — Print first N lines of a file (like head -n N)
//
// Usage: ex01_head <filename> [N]
//   N defaults to 10 if not specified.
//   Handle file not found with a friendly error message.

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse arguments
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: {} <filename> [lines]", args[0]);
        return;
    };

    let n: usize = if args.len() > 2 {
        match args[2].parse() {
            Ok(n) if n > 0 => n,
            _ => {
                eprintln!("Error: line count must be a positive number, got '{}'", args[2]);
                return;
            }
        }
    } else {
        10
    };

    // Read and print first N lines
    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut count = 0;

            for line in reader.lines() {
                match line {
                    Ok(text) => {
                        println!("{}", text);
                        count += 1;
                        if count >= n {
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading line: {}", e);
                        return;
                    }
                }
            }
        }
        Err(e) => {
            // Friendly error for file not found
            eprintln!("Error: could not open '{}'", filename);
            eprintln!("Reason: {}", e);
            eprintln!("Make sure the file exists and is readable.");
            return;
        }
    }
}
