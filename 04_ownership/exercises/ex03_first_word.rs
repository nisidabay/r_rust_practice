/*
 * ex03_first_word.rs — Exercise 3
 *
 * Task: Write fn first_word(s: &str) -> &str that returns a slice
 *       of the first word (up to first space). Handle empty string
 *       and single word (no spaces).
 *
 * Run: echo "hello world" | ./ex03_first_word
 * Expected: hello
 */

use std::io::{self, BufRead};

fn first_word(s: &str) -> &str {
    // Find first space; if none, return the whole string
    match s.find(' ') {
        Some(pos) => &s[..pos],
        None => s,
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            println!("(empty input)");
            continue;
        }
        println!("{}", first_word(&line));
    }
}
