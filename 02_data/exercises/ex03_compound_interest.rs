/*
 * ex03_compound_interest.rs — Exercise 3
 *
 * Task: Read principal, rate (%), and years from stdin.
 *       Print yearly table showing growth: A = P(1 + r)^n
 *
 * Run: echo -e "1000\n5\n10" | ./ex03_compound_interest
 * Expected: table of year + amount with 2 decimal places
 */

use std::io::{self, Write};

fn main() {
    print!("Enter principal: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let principal: f64 = input.trim().parse().expect("Invalid principal");

    print!("Enter rate (%): ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let rate: f64 = input.trim().parse().expect("Invalid rate");

    print!("Enter years: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let years: i32 = input.trim().parse().expect("Invalid years");

    let r = rate / 100.0;

    // Print table header
    println!("\n{:<6} {:>12}", "Year", "Amount");
    println!("{}", "------ ------------");

    let mut amount = principal;
    for year in 1..=years {
        amount = principal * (1.0 + r).powi(year);
        println!("{:<6} {:>12.2}", year, amount);
    }

    println!("\nAfter {} years: ${:.2}", years, amount);
}
