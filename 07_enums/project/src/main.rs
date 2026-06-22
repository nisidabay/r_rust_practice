// Project: CLI Dispatcher — todo list with enum-based command dispatch
//
// Reads commands like "add hello", "list", "done 2", "quit" from stdin.
// Uses an enum (Command) for clean dispatch — no if-else chain on strings.
// The enum ensures every command is explicitly handled (compiler checks).

use std::io::{self, BufRead, Write};

// --- Enums for command dispatch ---

#[derive(Debug)]
enum Command {
    Add(String),
    Done(usize),
    List,
    Quit,
    Help,
}

fn parse_command(line: &str) -> Command {
    let line = line.trim();
    let (cmd, arg) = match line.split_once(' ') {
        Some((c, a)) => (c, Some(a.trim())),
        None => (line, None),
    };

    match cmd {
        "add" | "a" => Command::Add(arg.unwrap_or("").to_string()),
        "done" | "d" => {
            let n: usize = arg.and_then(|a| a.parse().ok()).unwrap_or(0);
            Command::Done(n)
        }
        "list" | "ls" | "l" => Command::List,
        "quit" | "q" | "exit" => Command::Quit,
        _ => Command::Help,
    }
}

// --- Main ---

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut items: Vec<String> = Vec::new();

    println!("CLI Todo — type 'add buy milk', 'list', 'done 2', 'quit'");
    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut buf = String::new();
        if stdin.lock().read_line(&mut buf).is_err() {
            break;
        }
        let buf = buf.trim().to_string();
        if buf.is_empty() {
            continue;
        }

        match parse_command(&buf) {
            Command::Add(task) => {
                if task.is_empty() {
                    println!("Usage: add <task>");
                } else {
                    items.push(task);
                    println!("Added item #{}", items.len());
                }
            }
            Command::Done(n) => {
                if n == 0 || n > items.len() {
                    println!("Invalid item number (1..{})", items.len());
                } else {
                    let removed = items.remove(n - 1);
                    println!("Done: {}", removed);
                }
            }
            Command::List => {
                if items.is_empty() {
                    println!("No items.");
                } else {
                    for (i, item) in items.iter().enumerate() {
                        println!("{}. {}", i + 1, item);
                    }
                }
            }
            Command::Quit => {
                println!("Goodbye!");
                break;
            }
            Command::Help => {
                println!("Commands:");
                println!("  add <task>   — add a new todo item");
                println!("  done <n>     — mark item #n as done (removes it)");
                println!("  list         — show all items");
                println!("  help         — show this help");
                println!("  quit         — exit");
            }
        }
    }
}
