// ex04_bonus_find.rs — Recursively walk directory with std::fs::read_dir
//
// Print files whose name contains a given pattern (string contains).
// Recursively walks subdirectories.
//
// Usage: ex04_bonus_find <pattern> <directory>
//   ex04_bonus_find rs /home/nisidabay/r_rust_practice
//   ex04_bonus_find .txt .

use std::env;
use std::fs;
use std::path::Path;

fn find_files(dir: &Path, pattern: &str, depth: usize) -> Result<(), String> {
    // Avoid going too deep (safety limit)
    if depth > 20 {
        return Err("max depth (20) exceeded".to_string());
    }

    let entries =
        fs::read_dir(dir).map_err(|e| format!("Error reading directory {:?}: {}", dir, e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Error reading entry: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            // Recurse into subdirectories
            find_files(&path, pattern, depth + 1)?;
        } else if path.is_file() {
            // Check if filename contains the pattern
            if let Some(filename) = path.file_name() {
                let filename_str = filename.to_string_lossy();
                if filename_str.contains(pattern) {
                    println!("{}", path.display());
                }
            }
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let pattern = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: {} <pattern> [directory]", args[0]);
        eprintln!("  Pattern: string to search for in filenames");
        eprintln!("  Directory: start directory (default: current dir)");
        return;
    };

    let start_dir = if args.len() > 2 {
        Path::new(&args[2])
    } else {
        Path::new(".")
    };

    if !start_dir.exists() {
        eprintln!("Error: directory '{:?}' does not exist", start_dir);
        return;
    }
    if !start_dir.is_dir() {
        eprintln!("Error: '{:?}' is not a directory", start_dir);
        return;
    }

    println!("Searching for files matching '{}' in {:?}", pattern, start_dir);
    println!("{}", "-".repeat(60));

    match find_files(start_dir, pattern, 0) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
