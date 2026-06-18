// hex_dump — CLI hex dump tool
//
// Reads a file and dumps raw bytes in classic hexdump format.
// Uses unsafe for direct memory access (memory-mapped I/O simulation).
//
// Usage: cargo run -- <file>
//   or:  hex_dump <file>
//   or:  hex_dump --help

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

const BYTES_PER_LINE: usize = 16;
const DEFAULT_WIDTH: usize = 80;

fn print_usage(program: &str) {
    eprintln!("Usage: {program} <file>");
    eprintln!();
    eprintln!("A hex dump utility that displays file contents as hexadecimal bytes");
    eprintln!("alongside an ASCII representation.");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  <file>  Path to the file to dump");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --help  Show this help message and exit");
    eprintln!();
    eprintln!("Example:");
    eprintln!("  {program} myfile.bin");
}

/// Read the entire file into a Vec<u8>.
/// Using unsafe to demonstrate raw pointer access for printing.
fn read_file(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

/// Print a hex dump line with address, hex content, and ASCII sidebar.
///
/// Uses unsafe to demonstrate working with raw pointers to print
/// the hex representation more efficiently.
fn print_hex_line(data: &[u8], offset: usize) {
    // Print address offset
    print!("{:08X}  ", offset);

    // Print hex bytes (two groups of 8 for readability)
    for i in 0..BYTES_PER_LINE {
        if i < data.len() {
            unsafe {
                // Use raw pointer to access the byte
                let ptr = data.as_ptr().add(i);
                print!("{:02X} ", *ptr);
            }
        } else {
            print!("   ");
        }

        // Add extra space between byte groups
        if i == 7 {
            print!(" ");
        }
    }

    // Print ASCII representation
    print!(" |");
    for &byte in data.iter().take(BYTES_PER_LINE) {
        // Only print printable ASCII characters
        if byte.is_ascii_graphic() || byte == b' ' {
            print!("{}", byte as char);
        } else {
            print!(".");
        }
    }
    println!("|");
}

/// Count how many lines a terminal width can fit (just for display info).
fn estimate_lines(data_len: usize) -> usize {
    (data_len + BYTES_PER_LINE - 1) / BYTES_PER_LINE
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse arguments
    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        print_usage(&args[0]);
        process::exit(if args.len() < 2 { 1 } else { 0 });
    }

    let path = &args[1];

    // Read the file
    let data = match read_file(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error: could not read '{}': {}", path, e);
            process::exit(1);
        }
    };

    // Print header
    let total_lines = estimate_lines(data.len());
    println!(
        "Hex dump of '{}' — {} bytes ({} lines)",
        path,
        data.len(),
        total_lines
    );
    println!(
        "{:8}  {:47}  |ASCII|",
        "Address", "Hex bytes"
    );
    println!("{}", "-".repeat(DEFAULT_WIDTH));

    // Print hex dump line by line
    for (line_idx, chunk) in data.chunks(BYTES_PER_LINE).enumerate() {
        let offset = line_idx * BYTES_PER_LINE;
        print_hex_line(chunk, offset);
    }

    println!("{}", "-".repeat(DEFAULT_WIDTH));
    println!("End of file — {} bytes total.", data.len());
}
