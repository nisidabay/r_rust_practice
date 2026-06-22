/*
 * ex04_bonus_collatz.rs — Exercise 4 (BONUS)
 *
 * Task: Print the Collatz sequence starting from a number.
 *   If n is even → n = n / 2
 *   If n is odd  → n = 3n + 1
 *   Stop when n == 1. Count the steps.
 *
 * Run: ./ex04_bonus_collatz
 * Enter start: 13
 * Expected: 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1 (steps=9)
 */

use std::io::{self, Write};

fn main() {
    print!("Enter start: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: u64 = input.trim().parse().expect("Not a number");

    let mut steps = 0u32;
    print!("{}", n);

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        print!(" → {}", n);
        steps += 1;
    }
    println!(" (steps={})", steps);
}
