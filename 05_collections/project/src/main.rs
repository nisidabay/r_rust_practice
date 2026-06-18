// grocery_list — CLI grocery list manager
//
// Usage:
//   cargo run -- add milk        # Add "milk" to the list
//   cargo run -- add "ice cream" # Add "ice cream" (quoted for multi-word)
//   cargo run -- list            # Show all items
//   cargo run -- remove 3        # Remove item #3
//   cargo run -- clear           # Clear the entire list
//   cargo run -- help            # Show this help
//
// Internally uses Vec<String> to store items.

use std::env;
use std::fs;
use std::path::PathBuf;

const DATA_FILE: &str = "grocery_list.txt";

fn load_list() -> Vec<String> {
    let path = data_path();
    if !path.exists() {
        return Vec::new();
    }
    let content = fs::read_to_string(&path).unwrap_or_default();
    content.lines().map(|s| s.to_string()).collect()
}

fn save_list(items: &[String]) {
    let path = data_path();
    fs::write(&path, items.join("\n")).expect("Failed to save list");
}

fn data_path() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push(DATA_FILE);
    p
}

fn print_help(program: &str) {
    eprintln!("grocery_list — a simple CLI grocery list manager");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  {program} add <item>    Add an item to the list");
    eprintln!("  {program} list          Show all items");
    eprintln!("  {program} remove <n>    Remove item at position n (1-based)");
    eprintln!("  {program} clear         Remove all items");
    eprintln!("  {program} help          Show this help message");
}

fn cmd_list() {
    let items = load_list();
    if items.is_empty() {
        println!("Grocery list is empty.");
    } else {
        println!("Grocery list ({}/{}):", items.len(), items.len());
        for (i, item) in items.iter().enumerate() {
            println!("  {}. {}", i + 1, item);
        }
    }
}

fn cmd_add(name: &str) {
    let mut items = load_list();
    items.push(name.to_string());
    save_list(&items);
    println!("Added \"{name}\" to the list.");
}

fn cmd_remove(pos_str: &str) {
    let pos: usize = match pos_str.parse() {
        Ok(n) if n >= 1 => n,
        _ => {
            eprintln!("Error: \"{pos_str}\" is not a valid position number.");
            return;
        }
    };

    let mut items = load_list();
    if pos > items.len() {
        eprintln!(
            "Error: position {pos} is out of range. List has {} items.",
            items.len()
        );
        return;
    }

    let removed = items.remove(pos - 1);
    save_list(&items);
    println!("Removed \"{removed}\".");
}

fn cmd_clear() {
    let path = data_path();
    if path.exists() {
        fs::remove_file(&path).ok();
    }
    println!("Grocery list cleared.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = if args.is_empty() {
        "grocery_list"
    } else {
        &args[0]
    };

    // Strip path for display
    let program_name = program
        .split('/')
        .last()
        .unwrap_or(program)
        .split('\\')
        .last()
        .unwrap_or(program);

    if args.len() < 2 {
        print_help(program_name);
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Error: missing item name. Usage: {program_name} add <item>");
                return;
            }
            cmd_add(&args[2..].join(" "));
        }
        "list" => cmd_list(),
        "remove" => {
            if args.len() < 3 {
                eprintln!("Error: missing position. Usage: {program_name} remove <n>");
                return;
            }
            cmd_remove(&args[2]);
        }
        "clear" => cmd_clear(),
        "help" | "--help" | "-h" => print_help(program_name),
        other => {
            eprintln!("Unknown command: \"{other}\".");
            eprintln!("Run \"{program_name} help\" for usage.");
        }
    }
}
