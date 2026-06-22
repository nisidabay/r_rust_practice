/*
 * Project: wc — minimal word count clone
 *
 * Reads stdin until EOF, outputs: lines, words, characters.
 *
 * This is where ALL six concepts come together in one program:
 *   println!, variables, types, input, strings, arrays
 *
 * Usage:
 *   cargo run -- < file.txt
 *   echo "hello world" | cargo run --
 *
 * Example:
 *   echo "hello world" | cargo run --
 *   → 1 lines, 2 words, 12 chars
 */

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = 0u32;
    let mut words = 0u32;
    let mut chars = 0u32;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        lines += 1;
        words += line.split_whitespace().count() as u32;
        chars += line.len() as u32;
    }

    println!("{} lines, {} words, {} chars", lines, words, chars);
}
