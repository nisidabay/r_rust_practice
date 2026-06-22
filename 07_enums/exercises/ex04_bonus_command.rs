// ex04_bonus_command.rs — Enum Command with REPL loop
// A minimal CLI with: Quit, Help, Add(String), List
// Demonstrates how enums make command dispatch clean and safe

use std::io::{self, BufRead};

#[derive(Debug)]
enum Command {
    Quit,
    Help,
    Add(String),
    List,
}

// Parse a single line of user input into a Command
fn parse_command(line: &str) -> Command {
    let line = line.trim();
    if line.is_empty() {
        return Command::Help;
    }
    let (cmd, rest) = match line.split_once(' ') {
        Some((c, r)) => (c, Some(r.trim())),
        None => (line, None),
    };
    match cmd {
        "quit" | "q" | "exit" => Command::Quit,
        "help" | "h" | "?" => Command::Help,
        "add" | "a" => Command::Add(rest.unwrap_or("").to_string()),
        "list" | "ls" | "l" => Command::List,
        _ => Command::Help,
    }
}

fn main() {
    let stdin = io::stdin();
    let mut todo: Vec<String> = Vec::new();

    println!("Simple REPL — type 'add buy milk', 'list', 'quit'");
    loop {
        print!("> ");
        // Flush stdout so prompt appears immediately
        use std::io::Write;
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_err() || line.trim().is_empty() {
            // EOF or read error — treat as quit
            println!("Bye!");
            break;
        }

        match parse_command(&line) {
            Command::Quit => {
                println!("Bye!");
                break;
            }
            Command::Help => {
                println!("Commands:");
                println!("  add <item>  — add a todo item");
                println!("  list        — show all items");
                println!("  quit        — exit");
            }
            Command::Add(item) => {
                if item.is_empty() {
                    println!("Usage: add <item>");
                } else {
                    todo.push(item);
                    println!("Added: {}", todo.last().unwrap());
                }
            }
            Command::List => {
                if todo.is_empty() {
                    println!("No items.");
                } else {
                    for (i, item) in todo.iter().enumerate() {
                        println!("{}. {}", i + 1, item);
                    }
                }
            }
        }
    }
}
