/*
 * ex02_guessing.rs — Exercise 2
 *
 * Task: Guess a secret number (hardcoded). Loop until correct.
 *       Print "Warmer" or "Colder" hints based on distance.
 *
 * Run: ./ex02_guessing
 *       (enter guesses until you find the secret number)
 */

use std::io::{self, Write};

fn main() {
    // Secret number — using current time for variety
    let secret = 42; // hardcoded for determinism
    println!("I'm thinking of a number between 1 and 100...");
    println!("(it's {})", secret); // cheat line for testing

    let mut attempts = 0;
    let mut last_diff: Option<i32> = None;

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

        if guess == secret {
            println!("Correct! You got it in {} attempt(s)!", attempts);
            break;
        }

        let diff = (guess - secret).abs();
        match last_diff {
            None => {
                if diff <= 10 {
                    println!("Warm!");
                } else {
                    println!("Cold!");
                }
            }
            Some(prev) => {
                if diff < prev {
                    println!("Warmer!");
                } else if diff > prev {
                    println!("Colder!");
                } else {
                    println!("Same distance...");
                }
            }
        }
        last_diff = Some(diff);
    }
}
