// ex01_head.rs — Like `head -n 5` — read file and print first N lines
// Usage: ./ex01_head <file> [lines=5]
// Reads a file and prints the first N lines (default 5).

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn run_head(path: &str, n: usize) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        if i >= n {
            break;
        }
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let (path, n) = match args.len() {
        2 => (args[1].clone(), 5usize),
        3 => {
            let n = args[2].parse::<usize>()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput,
                    "line count must be a positive integer"))?;
            (args[1].clone(), n)
        }
        _ => {
            eprintln!("Usage: {} <file> [lines=5]", args.get(0).unwrap_or(&"ex01_head".into()));
            std::process::exit(1);
        }
    };

    run_head(&path, n)
}
