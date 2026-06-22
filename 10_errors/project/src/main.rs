// Project: file reader — read a file, print with line numbers
//
// Handles errors: file not found, permission denied, non-UTF8 content.
// Uses custom error type. Supports --head N and --tail N flags.
//
// Usage:
//   cargo run -- Cargo.toml
//   cargo run -- Cargo.toml --head 5
//   cargo run -- Cargo.toml --tail 3
//   cargo run -- nonexistent.txt

use std::env;
use std::fmt;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

// --- Custom error type ---

#[derive(Debug)]
enum FileError {
    NotFound(String),
    PermissionDenied(String),
    NotUtf8(String),
    IoError(io::Error),
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::NotFound(path) => write!(f, "file not found: {}", path),
            FileError::PermissionDenied(path) => write!(f, "permission denied: {}", path),
            FileError::NotUtf8(path) => write!(f, "non-UTF8 content: {}", path),
            FileError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for FileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FileError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for FileError {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::NotFound => FileError::NotFound("(path)".to_string()),
            io::ErrorKind::PermissionDenied => FileError::PermissionDenied("(path)".to_string()),
            _ => FileError::IoError(e),
        }
    }
}

// --- Config parsing ---

struct Config {
    path: String,
    head: Option<usize>,
    tail: Option<usize>,
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: file_reader <file> [--head N] [--tail N]".to_string());
    }

    let mut path = String::new();
    let mut head: Option<usize> = None;
    let mut tail: Option<usize> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--head" => {
                i += 1;
                if i < args.len() {
                    head = Some(args[i].parse().map_err(|_| "invalid --head value")?);
                }
            }
            "--tail" => {
                i += 1;
                if i < args.len() {
                    tail = Some(args[i].parse().map_err(|_| "invalid --tail value")?);
                }
            }
            s if s.starts_with("--") => {
                return Err(format!("unknown flag: {}", s));
            }
            _ => {
                if path.is_empty() {
                    path = args[i].clone();
                }
            }
        }
        i += 1;
    }

    if path.is_empty() {
        return Err("no file specified".to_string());
    }

    Ok(Config { path, head, tail })
}

// --- File reading ---

fn read_file_lines(path: &str) -> Result<Vec<String>, FileError> {
    let file_path = Path::new(path);

    // Check if file exists before trying to open
    if !file_path.exists() {
        return Err(FileError::NotFound(path.to_string()));
    }

    // Try to open (catches permission errors)
    let file = fs::File::open(file_path)?;

    // Read line by line (catches non-UTF8 at read_line level)
    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => lines.push(line),
            Err(_) => return Err(FileError::NotUtf8(path.to_string())),
        }
    }

    Ok(lines)
}

fn main() -> Result<(), String> {
    let config = parse_args()?;

    // Read the file (our custom error type is converted to Display string)
    let lines = read_file_lines(&config.path).map_err(|e| e.to_string())?;

    if lines.is_empty() {
        println!("(empty file)");
        return Ok(());
    }

    // Determine which lines to show
    let total = lines.len();
    let (start, end) = match (config.head, config.tail) {
        (Some(h), None) => (0, h.min(total)),
        (None, Some(t)) => (total.saturating_sub(t), total),
        (Some(h), Some(t)) if h <= t => (0, h.min(total)),          // head takes priority
        (Some(_h), Some(_t)) => (total.saturating_sub(t), total),   // tail if h > t
        (None, None) => (0, total),
    };

    // Calculate line number width for alignment
    let width = total.to_string().len().max(2);

    // Print lines
    for i in start..end {
        println!("{:width$}: {}", i + 1, lines[i], width = width);
    }

    // Show summary if we truncated
    if start > 0 || end < total {
        println!("--- {} of {} lines shown ---", end - start, total);
    }

    Ok(())
}
