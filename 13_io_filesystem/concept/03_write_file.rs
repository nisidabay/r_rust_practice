// 03_write_file.rs — Writing files to disk
// Uses: fs::write, File::create, write!, writeln!, append, OpenOptions

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let path = "/tmp/_demo_write.txt";

    // --- Method 1: fs::write (simplest) ---
    // Writes a byte slice or &str to a file. Overwrites existing content.
    fs::write(path, "Hello from fs::write!\n")?;
    println!("Created file with fs::write");

    // --- Method 2: File::create + write! / writeln! ---
    // File::create opens a file for writing (truncates if exists).
    let mut file = File::create(path)?;
    write!(file, "Line 1: {}\n", "formatted output")?;
    writeln!(file, "Line 2: {}", 42)?;
    writeln!(file, "Line 3: {}", true)?;
    println!("Wrote 3 lines with write!/writeln!");

    // --- Method 3: Appending ---
    // OpenOptions allows fine-grained control: read, write, append, create, truncate.
    let mut file = OpenOptions::new()
        .append(true)   // don't truncate, append instead
        .create(true)   // create if missing
        .open(path)?;
    writeln!(file, "Line 4: appended!")?;
    println!("Appended line 4");

    // --- Method 4: Write mode without truncate ---
    // Open existing file, write from start, keep existing content beyond.
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;
    // This overwrites bytes from position 0 but doesn't truncate
    write!(file, "OVERWRITE")?; // first 9 chars replaced
    println!("Overwrote first 9 bytes");

    // --- Method 5: Binary write ---
    let bin_path = "/tmp/_demo_binary.bin";
    let mut file = File::create(bin_path)?;
    let data: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    file.write_all(&data)?;
    println!("Wrote binary data: {:?}", data);

    // Cleanup
    fs::remove_file(path)?;
    fs::remove_file(bin_path)?;

    println!("All write methods completed successfully.");
    Ok(())
}
