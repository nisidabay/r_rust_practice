// ex03_file_tree.rs — Recursively list files in a directory tree
// Usage: ./ex03_file_tree [directory]
// Defaults to current directory. Shows file sizes and indented structure.

use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn walk_dir(path: &Path, prefix: &str) -> io::Result<()> {
    let entries = fs::read_dir(path)?;
    let mut entries: Vec<_> = entries.collect::<Result<Vec<_>, _>>()?;

    // Sort by name
    entries.sort_by_key(|e| e.file_name());

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last { "    " } else { "│   " };

        let path = entry.path();
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();

        if path.is_dir() {
            println!("{}{}{}/", prefix, connector, name_str);
            walk_dir(&path, &format!("{}{}", prefix, child_prefix))?;
        } else {
            let size = fs::metadata(&path)?.len();
            println!("{}{}{} ({} bytes)", prefix, connector, name_str, size);
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let dir = if args.len() >= 2 {
        args[1].as_str()
    } else {
        "."
    };

    let path = Path::new(dir);
    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory", dir);
        std::process::exit(1);
    }

    println!("{}", path.canonicalize()?.display());
    walk_dir(path, "")
}
