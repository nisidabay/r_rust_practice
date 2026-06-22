/*
 * ex02_string_reverse.rs — Exercise 2
 *
 * Task: Take a String, return the reversed String manually (chars loop).
 *       Write fn reverse_string(s: &str) -> String
 *
 * Run: echo "hello" | ./ex02_string_reverse
 * Expected: olleh
 */

use std::io::{self, BufRead};

fn reverse_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars().rev() {
        result.push(c);
    }
    result
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let reversed = reverse_string(&line);
        println!("{}", reversed);
    }
}
