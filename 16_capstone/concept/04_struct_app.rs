// 04_struct_app.rs
// Capstone — Struct App pattern with methods and main loop
//
// Concepts: Struct App { state, file_path }, mutable methods, menu loop
//
// Usage:
//   rustc 04_struct_app.rs && ./04_struct_app

use std::io::{self, Write};
use std::fmt;

// --- Config with lifetime (group 12 style) ---
#[derive(Debug)]
struct DisplayConfig<'a> {
    prefix: &'a str,
    show_index: bool,
}

// --- State ---

#[derive(Debug)]
struct TodoItem {
    id: u32,
    text: String,
    done: bool,
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.done { "[✓]" } else { "[ ]" };
        write!(f, "{} {} — {}", status, self.id, self.text)
    }
}

// --- Application struct ---

struct App {
    items: Vec<TodoItem>,
    next_id: u32,
    file_path: String,
    dirty: bool,
}

impl App {
    fn new(file_path: &str) -> Self {
        App {
            items: Vec::new(),
            next_id: 1,
            file_path: file_path.to_string(),
            dirty: false,
        }
    }

    fn add(&mut self, text: &str) {
        self.items.push(TodoItem {
            id: self.next_id,
            text: text.to_string(),
            done: false,
        });
        println!("Added [{}]: {}", self.next_id, text);
        self.next_id += 1;
        self.dirty = true;
    }

    fn done(&mut self, id: u32) -> Result<(), String> {
        let item = self
            .items
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| format!("No item with id {}", id))?;
        item.done = true;
        println!("Marked [{}] as done: {}", id, item.text);
        self.dirty = true;
        Ok(())
    }

    fn remove(&mut self, id: u32) -> Result<(), String> {
        let pos = self
            .items
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| format!("No item with id {}", id))?;
        let removed = self.items.remove(pos);
        println!("Removed [{}]: {}", removed.id, removed.text);
        self.dirty = true;
        Ok(())
    }

    fn list(&self, config: &DisplayConfig) {
        if self.items.is_empty() {
            println!("  (no items)");
            return;
        }
        for (i, item) in self.items.iter().enumerate() {
            let prefix = if config.show_index {
                format!("{}: ", i + 1)
            } else {
                String::new()
            };
            println!("{}{}{}", config.prefix, prefix, item);
        }
    }

    fn display_menu() {
        println!();
        println!("=== Todo App ===");
        println!("Commands:");
        println!("  add <text>      — Add a new todo");
        println!("  done <id>       — Mark todo as done");
        println!("  remove <id>     — Remove a todo");
        println!("  list            — List all todos");
        println!("  save            — Save to file");
        println!("  help            — Show this menu");
        println!("  quit            — Exit");
        println!();
    }

    fn run(&mut self) {
        App::display_menu();
        let config = DisplayConfig {
            prefix: "  ",
            show_index: true,
        };

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }
            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            let parts: Vec<&str> = input.splitn(2, ' ').collect();
            let command = parts[0];
            let rest = parts.get(1).map(|s| s.trim()).unwrap_or("");

            match command {
                "quit" | "exit" | "q" => {
                    if self.dirty {
                        println!("You have unsaved changes. Type 'save' to save or 'quit' again to discard.");
                        self.dirty = false; // prevent double prompt
                    }
                    println!("Goodbye!");
                    break;
                }
                "add" => {
                    if rest.is_empty() {
                        println!("Usage: add <text>");
                    } else {
                        self.add(rest);
                    }
                }
                "done" => {
                    if let Ok(id) = rest.parse::<u32>() {
                        if let Err(e) = self.done(id) {
                            println!("Error: {}", e);
                        }
                    } else {
                        println!("Usage: done <id>");
                    }
                }
                "remove" => {
                    if let Ok(id) = rest.parse::<u32>() {
                        if let Err(e) = self.remove(id) {
                            println!("Error: {}", e);
                        }
                    } else {
                        println!("Usage: remove <id>");
                    }
                }
                "list" => {
                    self.list(&config);
                }
                "save" => {
                    println!("Save would write to '{}'", self.file_path);
                    self.dirty = false;
                    println!("Saved.");
                }
                "help" => {
                    App::display_menu();
                }
                _ => {
                    println!("Unknown command: '{}'. Type 'help' for commands.", command);
                }
            }
        }
    }
}

fn main() {
    let mut app = App::new("todo_data.txt");
    // Add some sample items
    app.add("Learn Rust struct patterns");
    app.add("Build the capstone project");
    app.add("Write clean documentation");
    println!();
    println!("Current items:");
    app.list(&DisplayConfig {
        prefix: "  ",
        show_index: true,
    });
    println!();
    println!("Starting interactive mode (type 'quit' to exit)...");
    app.run();
}
