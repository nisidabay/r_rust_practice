// 02_read_file.rs — Reading files from disk
// Uses: fs::read_to_string, File::open, read_to_end, BufReader

use std::fs;
use std::io::{self, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    // Create a temp file so we can demonstrate reading.
    let path = Path::new("/tmp/_demo_read.txt");
    fs::write(path, "line one\nline two\nline three\n")?;

    // --- Method 1: fs::read_to_string (simplest) ---
    // Reads entire file into a String. Panics on non-UTF-8.
    let contents = fs::read_to_string(path)?;
    println!("=== read_to_string ===");
    println!("{}", contents);

    // --- Method 2: fs::read (binary) ---
    // Reads entire file into Vec<u8>. Good for non-text files.
    let bytes = fs::read(path)?;
    println!("=== fs::read (binary) ===");
    println!("Read {} bytes, first 4: {:?}", bytes.len(), &bytes[..4]);

    // --- Method 3: File::open + read_to_end ---
    // More explicit — gives you the File handle for seek, etc.
    let mut file = fs::File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    println!("=== File::open + read_to_end ===");
    println!("Read {} bytes", buf.len());

    // --- Method 4: Reading chunks (efficient for large files) ---
    let mut file = fs::File::open(path)?;
    let mut chunk = [0u8; 8]; // small buffer
    let n = file.read(&mut chunk)?;
    println!("=== Chunk read ===");
    println!("First chunk ({} bytes): {:?}", n, &chunk[..n]);

    // --- Method 5: Reading line by line (BufReader) ---
    // We'll cover BufReader properly in 04_buffered_io.rs
    let file = fs::File::open(path)?;
    use std::io::BufRead;
    let reader = std::io::BufReader::new(file);
    println!("=== Line by line ===");
    for line in reader.lines() {
        let line = line?;
        println!("  |{}|", line);
    }

    // Cleanup
    fs::remove_file(path)?;

    println!("All read methods completed successfully.");
    Ok(())
}
