// Interactive CLI math quiz with addition and multiplication questions.
// Usage: cargo run -- --difficulty easy|medium|hard
// The program loops through random questions, tracks score, and handles
// user input for answers.

use std::env;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

// A simple PRNG since we can't use external crates (rand).
// Uses a linear congruential generator with classic constants.
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new() -> Self {
        // Seed from system time (nanoseconds since epoch).
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as u64;
        SimpleRng {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    // Generate a pseudo-random u64 using LCG constants (Numerical Recipes).
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }

    // Generate a number in [lo, hi] inclusive.
    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        let range = (hi - lo + 1) as u64;
        lo + (self.next_u64() % range) as i32
    }
}

// Difficulty levels control the range of numbers in questions.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Difficulty {
    Easy,   // numbers 1..=9
    Medium, // numbers 1..=20
    Hard,   // numbers 1..=50
}

impl Difficulty {
    // Parse from string; case-insensitive.
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "easy" => Some(Difficulty::Easy),
            "medium" => Some(Difficulty::Medium),
            "hard" => Some(Difficulty::Hard),
            _ => None,
        }
    }

    fn max_num(&self) -> i32 {
        match self {
            Difficulty::Easy => 9,
            Difficulty::Medium => 20,
            Difficulty::Hard => 50,
        }
    }
}

// Represents a single quiz question.
enum Question {
    Addition { a: i32, b: i32 },
    Multiplication { a: i32, b: i32 },
}

impl Question {
    // Generate a random question at the given difficulty.
    fn random(rng: &mut SimpleRng, difficulty: Difficulty) -> Self {
        let max = difficulty.max_num();
        // Decide operation type: 0 = addition, 1 = multiplication.
        if rng.range(0, 1) == 0 {
            Question::Addition {
                a: rng.range(1, max),
                b: rng.range(1, max),
            }
        } else {
            Question::Multiplication {
                a: rng.range(1, max / 2),  // Keep products manageable
                b: rng.range(1, max / 2),
            }
        }
    }

    // Display the question to the user.
    fn display(&self) -> String {
        match self {
            Question::Addition { a, b } => format!("What is {a} + {b}?"),
            Question::Multiplication { a, b } => format!("What is {a} × {b}?"),
        }
    }

    // Return the correct answer.
    fn answer(&self) -> i32 {
        match self {
            Question::Addition { a, b } => a + b,
            Question::Multiplication { a, b } => a * b,
        }
    }
}

fn show_help() {
    println!("Usage: quiz --difficulty <easy|medium|hard>");
    println!();
    println!("Interactive CLI math quiz. Answer addition and multiplication");
    println!("questions at your chosen difficulty level.");
    println!();
    println!("Options:");
    println!("  --difficulty LEVEL  Set difficulty (easy, medium, hard)");
    println!("  --help              Show this help message");
    println!();
    println!("Difficulty levels:");
    println!("  easy    Numbers 1-9");
    println!("  medium  Numbers 1-20");
    println!("  hard    Numbers 1-50");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Simple argument parsing — no clap dependency.
    let mut difficulty = Difficulty::Easy; // default

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" => {
                show_help();
                return;
            }
            "--difficulty" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("Error: --difficulty requires a value (easy|medium|hard)");
                    std::process::exit(1);
                }
                match Difficulty::from_str(&args[i]) {
                    Some(d) => difficulty = d,
                    None => {
                        eprintln!(
                            "Error: unknown difficulty '{}'. Use easy, medium, or hard.",
                            args[i]
                        );
                        std::process::exit(1);
                    }
                }
            }
            _ => {
                eprintln!("Error: unknown argument '{}'. Use --help for usage.", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    println!("🧮 Rust Math Quiz");
    println!("Difficulty: {:?}", difficulty);
    println!("Answer 10 questions. Type your answer and press Enter.");
    println!("Type 'quit' to exit early.");
    println!();

    let mut rng = SimpleRng::new();
    let mut score = 0;
    let total_questions = 10;

    // Main game loop — 10 questions, then show score.
    for q_num in 1..=total_questions {
        let question = Question::random(&mut rng, difficulty);

        println!("Question {q_num}/{total_questions}: {}", question.display());

        // Inner loop for retrying on invalid input (but not on wrong answer).
        loop {
            print!("Your answer: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).unwrap_or(0) == 0 {
                // EOF (pipe closed) — exit gracefully.
                println!("\nGoodbye! Your final score: {score}/{total_questions}");
                return;
            }

            let trimmed = input.trim();

            // Check for quit command.
            if trimmed.eq_ignore_ascii_case("quit") {
                println!("Thanks for playing! Score: {score}/{total_questions}");
                return;
            }

            // Parse the answer.
            match trimmed.parse::<i32>() {
                Ok(user_answer) => {
                    if user_answer == question.answer() {
                        println!("✅ Correct!");
                        score += 1;
                    } else {
                        println!("❌ Wrong. The answer is {}.", question.answer());
                    }
                    break; // Move to the next question.
                }
                Err(_) => {
                    println!("Please enter a valid number or 'quit'.");
                    // Continue the inner loop to retry this question.
                }
            }
        }
        println!();
    }

    // Final score display.
    println!("=== Quiz Complete ===");
    println!("Score: {score}/{total_questions}");
    let percentage = (score as f64 / total_questions as f64) * 100.0;
    println!("Grade: {:.1}%", percentage);
    if percentage >= 90.0 {
        println!("Excellent work! 🏆");
    } else if percentage >= 70.0 {
        println!("Good job! Keep practicing.");
    } else {
        println!("Review the concepts and try again.");
    }
}
