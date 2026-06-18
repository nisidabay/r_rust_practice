// BONUS: ex05_tail_f.rs — Like `tail -f` — watch a file for new content
// Usage: ./ex05_tail_f <file>
// Prints existing content, then watches for appended lines (polls every 500ms).

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

fn tail_f(path: &Path) -> io::Result<()> {
    // First, print the entire file
    if path.exists() {
        let content = fs::read_to_string(path)?;
        print!("{}", content);
    }

    // Open and seek to end
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::End(0))?;

    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line)?;

        if bytes > 0 {
            print!("{}", line);
            io::stdout().flush()?;
        } else {
            // No new data — wait and try again
            thread::sleep(Duration::from_millis(500));
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args.get(0).unwrap_or(&"ex05_tail_f".into()));
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    if !path.exists() {
        eprintln!("Error: file '{}' does not exist", args[1]);
        std::process::exit(1);
    }

    println!("=== tail -f '{}' (Ctrl+C to stop) ===", args[1]);
    tail_f(path)
}
