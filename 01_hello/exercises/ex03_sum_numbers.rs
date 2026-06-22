/*
 * ex03_sum_numbers.rs — Exercise 3
 *
 * Task: Read numbers (one per line from stdin), print sum and average.
 * Stop on empty line or EOF.
 *
 * Run: echo -e "10\n20\n30\n" | ./ex03_sum_numbers
 * Expected:
 *   sum=60, count=3, avg=20.0
 */

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut sum = 0.0;
    let mut count = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        let n: f64 = line.parse().expect("Not a number");
        sum += n;
        count += 1;
    }

    if count > 0 {
        println!("sum={}, count={}, avg={:.1}", sum, count, sum / count as f64);
    }
}
