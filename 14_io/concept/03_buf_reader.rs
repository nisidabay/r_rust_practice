// 03_buf_reader.rs — BufReader for line-by-line reading
//
// Reading a file one byte at a time is slow. BufReader reads large chunks
// and hands out lines from an internal buffer. Far fewer syscalls.
//
// Essential for: large files, log processing, streaming data.

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Count lines, words, and chars in a file using BufReader.
/// This processes the file line-by-line without loading it entirely into memory.
fn file_stats<P: AsRef<Path>>(path: P) -> io::Result<(usize, usize, usize)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;

    // BufReader::lines() returns an iterator over Result<String>
    // Each line is a newly allocated String (the newline is stripped)
    for line in reader.lines() {
        let line = line?;  // propagate I/O errors
        lines += 1;
        words += line.split_whitespace().count();
        chars += line.len();  // byte count; for Unicode chars use chars().count()
    }

    Ok((lines, words, chars))
}

fn main() {
    // Read our own source file line by line
    let source = "03_buf_reader.rs";

    // Method 1: using the helper function
    match file_stats(source) {
        Ok((lines, words, chars)) => {
            println!("{} stats (via BufReader):", source);
            println!("  Lines: {}", lines);
            println!("  Words: {}", words);
            println!("  Chars: {}", chars);
        }
        Err(e) => {
            eprintln!("Error reading {}: {}", source, e);
            return;
        }
    }

    // Method 2: manual BufReader usage, reading lines with index
    println!("\nFirst 5 lines of this file:");
    let file = File::open(source).expect("Failed to open file");
    let reader = BufReader::new(file);

    // .lines() returns an iterator — we can use .enumerate() and .take()
    for (i, line) in reader.lines().enumerate().take(5) {
        match line {
            Ok(text) => println!("  {:3}: {}", i + 1, text),
            Err(e) => eprintln!("  Error: {}", e),
        }
    }
    // BufReader is dropped here, closing the file

    // Method 3: reading from stdin (useful for pipes)
    println!("\n--- Reading from stdin (pipe some text) ---");
    let stdin = io::stdin();
    let handle = stdin.lock();
    // BufReader is already used internally by stdin.lock()
    let mut count = 0;
    for line in handle.lines() {
        if let Ok(text) = line {
            count += 1;
            if count > 3 { break; }  // limit output
            println!("Stdin line {}: {}", count, text);
        }
    }

    println!("\nBufReader: efficient, line-by-line processing without loading the whole file.");
}
