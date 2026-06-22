/*
 * ex03_multiplication_table.rs — Exercise 3
 *
 * Task: Read N from stdin, print an N×N multiplication table.
 *
 * Run: echo "5" | ./ex03_multiplication_table
 * Expected: 5×5 grid with formatted columns
 */

use std::io::{self, Write};

fn main() {
    print!("Enter N: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().expect("Invalid number");

    if n <= 0 || n > 20 {
        println!("N must be between 1 and 20.");
        return;
    }

    // Calculate column width based on largest number
    let max_val = n * n;
    let width = (max_val as f64).log10() as usize + 1;

    // Print header row
    print!("{:>width$} ", "", width = width);
    for col in 1..=n {
        print!("{:>width$} ", col, width = width);
    }
    println!("");

    // Print table rows
    for row in 1..=n {
        print!("{:>width$} ", row, width = width);
        for col in 1..=n {
            print!("{:>width$} ", row * col, width = width);
        }
        println!("");
    }
}
