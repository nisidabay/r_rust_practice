// snippets — CLI snippet manager
// Save, read, search code snippets from a snippets directory.
//
// Usage:
//   cargo run -- save <name> <content>   — save a snippet
//   cargo run -- list                     — list all snippets
//   cargo run -- show <name>             — show a snippet
//   cargo run -- search <query>          — search snippets by content
//   cargo run -- remove <name>           — remove a snippet
//   cargo run -- --help                  — show help

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

/// Directory where snippets are stored.
const SNIPPETS_DIR: &str = "snippets";

/// Ensure the snippets directory exists.
fn ensure_dir() -> io::Result<PathBuf> {
    let dir = PathBuf::from(SNIPPETS_DIR);
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

/// Save a snippet: write content to a file named `<name>.txt`.
fn cmd_save(name: &str, content: &str) -> io::Result<()> {
    let dir = ensure_dir()?;
    let path = dir.join(format!("{}.txt", name));
    fs::write(&path, content)?;
    println!("Saved snippet '{}' ({} bytes)", name, content.len());
    Ok(())
}

/// List all saved snippets with sizes.
fn cmd_list() -> io::Result<()> {
    let dir = ensure_dir()?;
    let entries = fs::read_dir(&dir)?;

    let mut snippets: BTreeMap<String, u64> = BTreeMap::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("txt") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                let size = fs::metadata(&path)?.len();
                snippets.insert(stem.to_string(), size);
            }
        }
    }

    if snippets.is_empty() {
        println!("No snippets found.");
        println!("  Use: cargo run -- save <name> <content>");
        return Ok(());
    }

    println!("=== Snippets ({} total) ===", snippets.len());
    for (name, size) in &snippets {
        println!("  {} ({} bytes)", name, size);
    }
    Ok(())
}

/// Show a snippet's content.
fn cmd_show(name: &str) -> io::Result<()> {
    let dir = ensure_dir()?;
    let path = dir.join(format!("{}.txt", name));

    if !path.exists() {
        eprintln!("Snippet '{}' not found.", name);
        std::process::exit(1);
    }

    let content = fs::read_to_string(&path)?;
    println!("=== {} ===", name);
    print!("{}", content);
    // Ensure trailing newline for display
    if !content.ends_with('\n') {
        println!();
    }
    Ok(())
}

/// Search snippets by content (case-insensitive substring match).
fn cmd_search(query: &str) -> io::Result<()> {
    let dir = ensure_dir()?;
    let entries = fs::read_dir(&dir)?;
    let query_lower = query.to_lowercase();

    let mut found = false;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("txt") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                let file = fs::File::open(&path)?;
                let reader = BufReader::new(file);
                for (lineno, line) in reader.lines().enumerate() {
                    let line = line?;
                    if line.to_lowercase().contains(&query_lower) {
                        if !found {
                            println!("=== Results for '{}' ===", query);
                            found = true;
                        }
                        println!("  {}:{}  {}", stem, lineno + 1, line.trim());
                    }
                }
            }
        }
    }

    if !found {
        println!("No matches found for '{}'.", query);
    }
    Ok(())
}

/// Remove a snippet.
fn cmd_remove(name: &str) -> io::Result<()> {
    let dir = ensure_dir()?;
    let path = dir.join(format!("{}.txt", name));

    if !path.exists() {
        eprintln!("Snippet '{}' not found.", name);
        std::process::exit(1);
    }

    fs::remove_file(&path)?;
    println!("Removed snippet '{}'.", name);
    Ok(())
}

fn print_usage(prog: &str) {
    eprintln!("Usage:");
    eprintln!("  {} save <name> <content>   — save a snippet", prog);
    eprintln!("  {} list                     — list all snippets", prog);
    eprintln!("  {} show <name>             — show a snippet", prog);
    eprintln!("  {} search <query>          — search snippets", prog);
    eprintln!("  {} remove <name>           — remove a snippet", prog);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let prog = args.get(0).map(|s| s.as_str()).unwrap_or("snippets");

    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("--help");

    match cmd {
        "save" => {
            let name = args.get(2).unwrap_or_else(|| {
                eprintln!("Error: missing <name>");
                print_usage(prog);
                std::process::exit(1);
            });
            let content = args.get(3).unwrap_or_else(|| {
                eprintln!("Error: missing <content>");
                print_usage(prog);
                std::process::exit(1);
            });
            cmd_save(name, content)?;
        }
        "list" => {
            cmd_list()?;
        }
        "show" => {
            let name = args.get(2).unwrap_or_else(|| {
                eprintln!("Error: missing <name>");
                print_usage(prog);
                std::process::exit(1);
            });
            cmd_show(name)?;
        }
        "search" => {
            let query = args.get(2).unwrap_or_else(|| {
                eprintln!("Error: missing <query>");
                print_usage(prog);
                std::process::exit(1);
            });
            cmd_search(query)?;
        }
        "remove" => {
            let name = args.get(2).unwrap_or_else(|| {
                eprintln!("Error: missing <name>");
                print_usage(prog);
                std::process::exit(1);
            });
            cmd_remove(name)?;
        }
        "--help" | "-h" => {
            print_usage(prog);
        }
        other => {
            eprintln!("Error: unknown command '{}'", other);
            print_usage(prog);
            std::process::exit(1);
        }
    }

    Ok(())
}
