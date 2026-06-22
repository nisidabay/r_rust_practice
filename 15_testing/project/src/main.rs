// Binary: CLI REPL calculator reading expressions
//
// Usage:
//   cargo run -- 3 + 4
//   cargo run -- 10 / 2
//   cargo run (interactive REPL)

use calc::{CalcError, Calculator};
use std::io::{self, BufRead, Write};

fn main() {
    let mut calc = Calculator::new();
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        // Single-expression mode
        let expr = args[1..].join(" ");
        match calc.eval_expression(&expr) {
            Ok(result) => println!("= {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
        return;
    }

    // REPL mode
    println!("calc REPL — type expressions like '3 + 4'");
    println!("Special commands: memory, store <n>, recall, clear");
    println!("Ctrl+D to quit.\n");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => {
                println!("\nGoodbye!");
                break;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match line {
            "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "memory" | "m" => {
                println!("Memory: {}", calc.recall());
            }
            "clear" | "c" => {
                calc.clear_memory();
                println!("Memory cleared.");
            }
            "recall" | "r" => {
                println!("= {}", calc.recall());
            }
            s if s.starts_with("store ") || s.starts_with("s ") => {
                let val_str = s.split_once(' ').map(|(_, v)| v).unwrap_or("");
                match val_str.parse::<f64>() {
                    Ok(n) => {
                        calc.store(n);
                        println!("Stored: {}", n);
                    }
                    Err(_) => eprintln!("Error: invalid number '{}'", val_str),
                }
            }
            s if s.starts_with("add ") || s.starts_with("a ") => {
                let val_str = s.split_once(' ').map(|(_, v)| v).unwrap_or("");
                match val_str.parse::<f64>() {
                    Ok(n) => {
                        calc.add_to_memory(n);
                        println!("Memory: {}", calc.recall());
                    }
                    Err(_) => eprintln!("Error: invalid number '{}'", val_str),
                }
            }
            expr => match calc.eval_expression(expr) {
                Ok(result) => println!("= {}", result),
                Err(e) => eprintln!("Error: {}", e),
            },
        }
    }
}
