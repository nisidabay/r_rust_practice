/*
 * ex02_line_stats.rs — Exercise 2
 *
 * Task: Read lines from stdin. For each line, print:
 *   line number, character count, word count
 *
 * Run: echo -e "hello world\nrust is fun" | ./ex02_line_stats
 * Expected:
 *   1: chars=11 words=2
 *   2: chars=13 words=3
 */

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for (i, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();
        let words = line.split_whitespace().count();
        println!("{}: chars={} words={}", i + 1, line.len(), words);
    }
}
