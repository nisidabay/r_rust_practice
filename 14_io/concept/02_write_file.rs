// 02_write_file.rs — Writing files: fs::write, File::create, append, write!
//
// Three common patterns: create new file (overwrite), append to existing,
// and use write! macro for formatted output.

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;

fn main() {
    let path = "/tmp/14_io_write_test.txt";

    // Method 1: fs::write — simplest, overwrites file
    // Creates the file if it doesn't exist, truncates if it does
    fs::write(path, "Line 1: Hello from fs::write!\n")
        .expect("Failed to write file");
    println!("Created {path}");

    // Verify by reading it back
    let contents = fs::read_to_string(path).unwrap();
    println!("Contents: {:?}", contents.trim());

    // Method 2: File::create — overwrites, returns a File handle
    // Useful when you need to write incrementally
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(b"Line A: Created with File::create\n")
        .expect("Failed to write");
    file.write_all(b"Line B: Second write to same file\n")
        .expect("Failed to write");
    // File is automatically closed (dropped) when it goes out of scope
    println!("Re-created with File::create");

    // Method 3: Append mode — doesn't overwrite
    let mut file = OpenOptions::new()
        .append(true)  // don't truncate, write at end
        .open(path)
        .expect("Failed to open for append");
    file.write_all(b"Line C: Appended!\n")
        .expect("Failed to append");
    println!("Appended a line");

    // Method 4: write! macro for formatted output
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)
        .expect("Failed to open for append");
    let name = "Rust";
    let year = 2024;
    // write! returns io::Result, so we need to handle it
    writeln!(file, "Formatted: {name} in {year}!").expect("Failed to write formatted");
    println!("Wrote formatted line");

    // Read final content
    let final_content = fs::read_to_string(path).unwrap();
    println!("\nFinal file contents:");
    for (i, line) in final_content.lines().enumerate() {
        println!("  {}: {}", i + 1, line);
    }

    // Cleanup
    // fs::remove_file(path).ok();  // uncomment to delete temp file
}
