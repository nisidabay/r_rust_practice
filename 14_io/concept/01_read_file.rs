// 01_read_file.rs — Reading files: read_to_string and File::open
//
// Two approaches: read entire file at once (simple) or open a File handle (flexible).
// Both return Result — you must handle the error case.

use std::fs;
use std::fs::File;
use std::io::Read;

fn main() {
    // Approach 1: read_to_string — simplest, loads whole file into memory
    // Best for: small to medium files (configs, logs under 100MB)
    let path = "Cargo.toml";
    match fs::read_to_string(path) {
        Ok(contents) => {
            // Use .lines() to iterate line by line without splitting
            let line_count = contents.lines().count();
            println!("Read Cargo.toml ({} lines):", line_count);
            for (i, line) in contents.lines().enumerate().take(3) {
                println!("  {}: {}", i + 1, line);
            }
            if line_count > 3 {
                println!("  ... ({} more lines)", line_count - 3);
            }
        }
        Err(e) => {
            eprintln!("Error reading {}: {}", path, e);
        }
    }

    // Approach 2: File::open + read_to_string — more control
    // You can seek, read partial contents, or check metadata before reading
    let path2 = "Cargo.toml";
    match File::open(path2) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(bytes_read) => {
                    println!("\nFile::open approach: read {} bytes", bytes_read);
                    println!("First line: {:?}", contents.lines().next().unwrap_or(""));
                }
                Err(e) => {
                    eprintln!("Error reading contents: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
        }
    }

    // Shorthand: use ? in main by making it return Result (but we don't here)
    // to keep it simple. See 03_result_test in group 15 for that pattern.

    println!("\nFile reading: always handle the Result!");
}
