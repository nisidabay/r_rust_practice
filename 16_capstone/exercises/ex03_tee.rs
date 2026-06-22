// ex03_tee.rs
// tee clone — read stdin, write to file AND stdout
//
// Features:
//   - Read from stdin, write to stdout and file simultaneously
//   - -a flag: append to file instead of overwrite
//   - Handle write errors gracefully
//
// Usage:
//   echo "Hello, world!" | rustc ex03_tee.rs && ./ex03_tee output.txt
//   echo "Another line" | ./ex03_tee -a output.txt
//   ls -la | ./ex03_tee listing.txt

use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Write};

struct TeeConfig {
    file_path: String,
    append: bool,
}

fn parse_args() -> Result<TeeConfig, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: ex03_tee [-a] <file>".to_string());
    }

    let mut file_path = String::new();
    let mut append = false;

    for arg in &args[1..] {
        match arg.as_str() {
            "-a" => append = true,
            f => {
                if file_path.is_empty() {
                    file_path = f.to_string();
                } else {
                    return Err(format!("Unexpected argument: {}", f));
                }
            }
        }
    }

    if file_path.is_empty() {
        return Err("No file specified".to_string());
    }

    Ok(TeeConfig { file_path, append })
}

fn main() -> Result<(), String> {
    let config = parse_args()?;

    // Open file for writing (append or overwrite)
    let file: File = if config.append {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&config.file_path)
            .map_err(|e| format!("Cannot open '{}' for append: {}", config.file_path, e))?
    } else {
        File::create(&config.file_path)
            .map_err(|e| format!("Cannot create '{}': {}", config.file_path, e))?
    };

    let mut file_writer = io::BufWriter::new(file);
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line_result in reader.lines() {
        let line = line_result.map_err(|e| format!("Read error: {}", e))?;

        // Write to stdout
        println!("{}", line);

        // Write to file
        writeln!(file_writer, "{}", line)
            .map_err(|e| format!("Write error to '{}': {}", config.file_path, e))?;
    }

    // Flush the file writer to ensure all data is written
    file_writer
        .flush()
        .map_err(|e| format!("Flush error: {}", e))?;

    Ok(())
}
