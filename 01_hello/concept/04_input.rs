/*
 * 04_input.rs — Practical Rust
 *
 * Question: How do I read from the keyboard?
 *
 * Use std::io::stdin() and read_line:
 *   let mut buf = String::new();
 *   io::stdin().read_line(&mut buf).expect("fail");
 *
 * parse() converts a string to a number:
 *   let n: i32 = buf.trim().parse().expect("not a number");
 *
 * Always .trim() the result — read_line includes the trailing '\n'!
 */

use std::io::{self, Write};

fn main() {
    // --- Read a string ---
    print!("Enter your name: ");
    io::stdout().flush().unwrap();   // print! buffers, force flush

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    // .trim() removes \n and trailing whitespace
    let name = name.trim();

    println!("Hello, {}!", name);

    // --- Read a number ---
    print!("Enter your age: ");
    io::stdout().flush().unwrap();

    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read line");

    // .trim() → remove \n, .parse() → string → number
    let age: i32 = age_input.trim().parse().expect("That's not a number!");
    println!("Next year you'll be {}", age + 1);

    // --- Read multiple values ---
    print!("Enter two numbers (space separated): ");
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");

    // Split by whitespace and parse each
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.len() >= 2 {
        let a: i32 = parts[0].parse().expect("Not a number");
        let b: i32 = parts[1].parse().expect("Not a number");
        println!("{} + {} = {}", a, b, a + b);
    }

    // --- Loop reading until EOF (Ctrl+D) ---
    println!("\nEnter lines (Ctrl+D to stop):");
    let mut total = 0i32;
    loop {
        let mut buf = String::new();
        let bytes_read = io::stdin().read_line(&mut buf).unwrap();
        if bytes_read == 0 {
            break;  // EOF
        }
        total += 1;
        print!("[{}] {}", total, buf);
    }
    println!("\nYou entered {} lines", total);
}
