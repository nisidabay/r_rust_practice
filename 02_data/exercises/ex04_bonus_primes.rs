/*
 * ex04_bonus_primes.rs — Bonus Exercise
 *
 * Task: Read N from stdin, print all prime numbers up to N using trial division.
 *       Also print how many primes were found.
 *
 * Run: echo "50" | ./ex04_bonus_primes
 * Expected: primes up to 50, count of how many
 */

use std::io::{self, Write};

fn main() {
    print!("Enter N: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().expect("Invalid number");

    if n < 2 {
        println!("No primes less than 2.");
        return;
    }

    let mut count = 0;
    println!("Primes up to {}: ", n);

    // Trial division — check each number from 2 to n
    for candidate in 2..=n {
        let mut is_prime = true;
        // Check divisibility up to sqrt(candidate)
        let limit = (candidate as f64).sqrt() as i32;
        for divisor in 2..=limit {
            if candidate % divisor == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            print!("{} ", candidate);
            count += 1;
        }
    }

    println!("\n\nFound {} primes.", count);
}
