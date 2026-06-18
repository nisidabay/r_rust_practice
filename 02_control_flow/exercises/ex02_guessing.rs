// Simple number guessing game using stdin, loop, and match.
// The program picks a random number, and the user guesses until correct.
// We use loop (infinite), match on Ordering, and basic input handling.

use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    // Pick a secret number between 1 and 100.
    // We use a simple LCG seeded with epoch time since Rust std lacks random.
    let secret = ((std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos()
        % 100)
        + 1) as i32;

    println!("I'm thinking of a number between 1 and 100.");
    println!("Can you guess it?");

    // Loop until the user guesses correctly.
    loop {
        print!("Your guess: ");
        // Flush stdout so the prompt appears before waiting for input.
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        // read_line returns Ok(0) on EOF (pipe closed). Exit gracefully.
        if io::stdin().read_line(&mut guess).unwrap_or(0) == 0 {
            println!("Goodbye!");
            break;
        }

        // Parse the input; invalid inputs just continue the loop.
        // `trim()` removes the trailing newline; `parse()` handles the rest.
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Match the comparison result to provide feedback.
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You got it! The number was {secret}.");
                break; // Exit the loop — game over.
            }
        }
    }
}
