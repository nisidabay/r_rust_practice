// ex02_config_reader.rs — Read and parse a config file with key=value pairs
// Usage: ./ex02_config_reader <config_file>
// Parses lines like "KEY = VALUE" and prints the resulting map.
// Ignores empty lines and lines starting with #.

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parse_config(path: &str) -> io::Result<HashMap<String, String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut config = HashMap::new();

    for (lineno, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Split on first '='
        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim();
            let value = line[eq_pos + 1..].trim();
            if key.is_empty() {
                eprintln!("Warning: line {} has empty key, skipping", lineno + 1);
                continue;
            }
            config.insert(key.to_string(), value.to_string());
        } else {
            eprintln!("Warning: line {} has no '=', skipping", lineno + 1);
        }
    }

    Ok(config)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <config_file>", args.get(0).unwrap_or(&"ex02_config_reader".into()));
        std::process::exit(1);
    }

    let config = parse_config(&args[1])?;

    println!("=== Parsed Config ===");
    let mut keys: Vec<&String> = config.keys().collect();
    keys.sort();
    for key in keys {
        println!("  {} = {}", key, config[key]);
    }
    println!("({} entries)", config.len());

    Ok(())
}
