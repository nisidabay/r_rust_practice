// ex02_cat.rs
// cat clone with line numbers and binary file detection
//
// Features:
//   - Read files and print to stdout (like cat)
//   - -n flag: number output lines
//   - Binary file detection (check first bytes for null byte)
//   - Handle multiple files
//
// Usage:
//   rustc ex02_cat.rs && ./ex02_cat file1.txt file2.txt -n
//   ./ex02_cat some_binary_file.bin

use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

struct CatConfig {
    files: Vec<String>,
    number_lines: bool,
}

fn parse_args() -> Result<CatConfig, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: ex02_cat <file...> [-n]".to_string());
    }

    let mut files = Vec::new();
    let mut number_lines = false;

    for arg in &args[1..] {
        match arg.as_str() {
            "-n" => number_lines = true,
            f => files.push(f.to_string()),
        }
    }

    if files.is_empty() {
        return Err("No files specified".to_string());
    }

    Ok(CatConfig { files, number_lines })
}

/// Check if content appears to be binary (has null bytes in first 512 bytes)
fn is_binary(data: &[u8]) -> bool {
    let check_len = data.len().min(512);
    data[..check_len].contains(&0x00)
}

/// Safely read a file, handling binary detection
fn read_file_safe(path: &str) -> Result<(Vec<u8>, bool), String> {
    let path_obj = Path::new(path);
    if !path_obj.exists() {
        return Err(format!("File not found: '{}'", path));
    }

    let mut file = fs::File::open(path_obj).map_err(|e| format!("Cannot open '{}': {}", path, e))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .map_err(|e| format!("Error reading '{}': {}", path, e))?;

    let binary = is_binary(&data);
    Ok((data, binary))
}

fn main() -> Result<(), String> {
    let config = parse_args()?;
    let mut line_counter = 1;

    for file_path in &config.files {
        let (data, is_binary) = read_file_safe(file_path)?;

        if is_binary {
            println!("{}: binary file — skipping", file_path);
            continue;
        }

        // Convert to string (may have non-UTF8 bytes, so we use lossy)
        let content = String::from_utf8_lossy(&data);

        if config.number_lines {
            for line in content.lines() {
                println!("{:>6}\t{}", line_counter, line);
                line_counter += 1;
            }
        } else {
            print!("{}", content);
        }
    }

    Ok(())
}
