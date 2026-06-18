//! todo — A simple CLI todo app with file persistence.
//!
//! Uses `Result` throughout for all fallible operations.
//!
//! Usage:
//!   cargo run -- add "Buy milk"
//!   cargo run -- list
//!   cargo run -- done 2
//!   cargo run -- rm 2
//!   cargo run -- help

use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

const DATA_FILE: &str = "todos.txt";

/// A single todo item.
#[derive(Debug)]
struct Todo {
    id: usize,
    text: String,
    done: bool,
}

// ─── File persistence ────────────────────────────────────────────

/// Read todos from the data file. Returns empty vec if file doesn't exist.
fn read_todos() -> Result<Vec<Todo>, String> {
    let path = Path::new(DATA_FILE);
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = fs::File::open(path).map_err(|e| format!("failed to open {DATA_FILE}: {e}"))?;
    let reader = io::BufReader::new(file);
    let mut todos = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|e| format!("failed to read line: {e}"))?;
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }
        // Format: "id|done|text"  (done is "1" or "0")
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() < 3 {
            continue; // skip malformed lines
        }
        let id: usize = parts[0]
            .parse()
            .map_err(|e| format!("corrupt data: bad id '{0}': {1}", parts[0], e))?;
        let done = parts[1] == "1";
        let text = parts[2].to_string();
        todos.push(Todo { id, text, done });
    }

    Ok(todos)
}

/// Write todos to the data file, overwriting completely.
fn write_todos(todos: &[Todo]) -> Result<(), String> {
    let mut file =
        fs::File::create(DATA_FILE).map_err(|e| format!("failed to write {DATA_FILE}: {e}"))?;

    for todo in todos {
        let done_flag = if todo.done { "1" } else { "0" };
        writeln!(file, "{}|{}|{}", todo.id, done_flag, todo.text)
            .map_err(|e| format!("failed to write todo: {e}"))?;
    }

    Ok(())
}

/// Generate the next available ID (max existing + 1, or 1 if empty).
fn next_id(todos: &[Todo]) -> usize {
    todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

// ─── Commands ────────────────────────────────────────────────────

fn cmd_add(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("usage: todo add <description>".to_string());
    }
    let text = args.join(" ");

    let mut todos = read_todos()?;
    let id = next_id(&todos);
    todos.push(Todo {
        id,
        text,
        done: false,
    });
    write_todos(&todos)?;
    println!("Added todo #{id}");
    Ok(())
}

fn cmd_list() -> Result<(), String> {
    let todos = read_todos()?;
    if todos.is_empty() {
        println!("No todos yet. Add one with: cargo run -- add \"your task\"");
        return Ok(());
    }
    for todo in &todos {
        let status = if todo.done { "[x]" } else { "[ ]" };
        println!("{}. {} {}", todo.id, status, todo.text);
    }
    Ok(())
}

fn cmd_done(args: &[String]) -> Result<(), String> {
    let id: usize = args
        .first()
        .ok_or_else(|| "usage: todo done <id>".to_string())
        .and_then(|s| s.parse().map_err(|_| format!("invalid id: {s}")))?;

    let mut todos = read_todos()?;
    let todo = todos
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| format!("no todo with id {id}"))?;
    todo.done = true;
    write_todos(&todos)?;
    println!("Marked todo #{id} as done");
    Ok(())
}

fn cmd_rm(args: &[String]) -> Result<(), String> {
    let id: usize = args
        .first()
        .ok_or_else(|| "usage: todo rm <id>".to_string())
        .and_then(|s| s.parse().map_err(|_| format!("invalid id: {s}")))?;

    let mut todos = read_todos()?;
    let before = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() == before {
        return Err(format!("no todo with id {id}"));
    }
    write_todos(&todos)?;
    println!("Removed todo #{id}");
    Ok(())
}

fn cmd_help() -> Result<(), String> {
    println!("Usage:");
    println!("  cargo run -- add <description>   Add a new todo");
    println!("  cargo run -- list                List all todos");
    println!("  cargo run -- done <id>           Mark a todo as done");
    println!("  cargo run -- rm <id>             Remove a todo");
    println!("  cargo run -- help                Show this help");
    Ok(())
}

// ─── Entry point ─────────────────────────────────────────────────

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        return cmd_list();
    }

    match args[0].as_str() {
        "add" => cmd_add(&args[1..]),
        "list" => cmd_list(),
        "done" => cmd_done(&args[1..]),
        "rm" => cmd_rm(&args[1..]),
        "help" | "--help" | "-h" => cmd_help(),
        other => Err(format!(
            "unknown command: {other}\nRun 'cargo run -- help' for usage."
        )),
    }
}
