/*
 * Project: Guessing Game — number guessing game
 *
 * Secret number from std::time::SystemTime (nanosecond entropy).
 * Loop until guess correct. Print "Too high" / "Too low".
 * Track attempts. Print stats at end.
 *
 * Usage:
 *   cargo run --
 *
 * Example:
 *   (interactive — enter guesses until correct)
 */

use std::io::{self, Write};

fn main() {
    // Generate secret using SystemTime as entropy source
    let secret: i32 = {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        // Map to 1..=100
        (nanos % 100) + 1
    };

    println!("I'm thinking of a number between 1 and 100...");
    println!("(secret is {})", secret); // cheat line for testing

    let mut attempts = 0;

    loop {
        print!("Your guess: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let guess: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };

        attempts += 1;

        if guess < secret {
            println!("Too low");
        } else if guess > secret {
            println!("Too high");
        } else {
            println!("Correct! You got it in {} attempt(s)!", attempts);
            break;
        }
    }

    // Stats
    println!();
    println!("=== Stats ===");
    println!("Secret number: {}", secret);
    println!("Total attempts: {}", attempts);
    if attempts == 1 {
        println!("Incredible — first try!");
    } else if attempts <= 5 {
        println!("Great job — very efficient!");
    } else if attempts <= 10 {
        println!("Good effort!");
    } else {
        println!("You got there eventually!");
    }
}
