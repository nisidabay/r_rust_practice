// ex01_filter_map.rs — Read numbers from stdin, filter evens, map to "even: N"
//
// Use an iterator chain on stdin lines. Parse numbers, filter even ones,
// map each to a formatted string, and print them.
//
// Input: one number per line (stop with Ctrl+D)
// Output: "even: N" for each even number

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    // Chain: read lines → filter valid → parse → filter evens → format → print
    let evens: Vec<String> = lines
        .filter_map(|line| line.ok())                    // ignore read errors
        .filter_map(|line| line.trim().parse::<i32>().ok())  // parse and skip non-numbers
        .filter(|n| n % 2 == 0)                            // keep only evens
        .map(|n| format!("even: {}", n))                   // format
        .collect();

    if evens.is_empty() {
        println!("No even numbers found.");
    } else {
        for e in &evens {
            println!("{}", e);
        }
        println!("Total evens: {}", evens.len());
    }
}
