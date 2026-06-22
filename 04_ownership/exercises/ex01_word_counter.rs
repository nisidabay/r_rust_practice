/*
 * ex01_word_counter.rs — Exercise 1
 *
 * Task: Read lines from stdin, count total words, return count from fn.
 *       Write a function count_words(text: &str) -> usize.
 *
 * Run: echo "hello world foo bar" | ./ex01_word_counter
 * Expected: 4
 */

use std::io::{self, BufRead};

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn main() {
    let stdin = io::stdin();
    let mut total = 0usize;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        total += count_words(&line);
    }

    println!("{}", total);
}
