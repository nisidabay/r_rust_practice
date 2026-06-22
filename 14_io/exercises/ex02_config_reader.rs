// ex02_config_reader.rs — Read KEY=VALUE config file into HashMap
//
// Format: one KEY=VALUE per line. Lines starting with # are comments.
// Blank lines are skipped. Handle:
//   - Missing file (warn and use defaults)
//   - Duplicate keys (silently overwrite or warn)
//   - Malformed lines (report and skip)

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Parse a config file into a HashMap.
/// Returns (config_map, errors_found)
fn parse_config<P: AsRef<Path>>(path: P) -> (HashMap<String, String>, Vec<String>) {
    let mut config = HashMap::new();
    let mut errors = Vec::new();

    let file = match File::open(path.as_ref()) {
        Ok(f) => f,
        Err(e) => {
            errors.push(format!("Warning: could not open {:?} — {}", path.as_ref(), e));
            return (config, errors);
        }
    };

    let reader = BufReader::new(file);
    for (lineno, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                errors.push(format!("Line {}: read error — {}", lineno + 1, e));
                continue;
            }
        };

        let trimmed = line.trim();

        // Skip blank lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Parse KEY=VALUE — find the first '='
        match trimmed.find('=') {
            Some(sep) => {
                let key = trimmed[..sep].trim().to_string();
                let value = trimmed[sep + 1..].trim().to_string();

                if key.is_empty() {
                    errors.push(format!("Line {}: empty key (malformed: {:?})", lineno + 1, trimmed));
                    continue;
                }

                // Warn on duplicate but allow overwrite
                if config.contains_key(&key) {
                    errors.push(format!("Warning: duplicate key '{}' on line {}, overwriting", key, lineno + 1));
                }

                config.insert(key, value);
            }
            None => {
                errors.push(format!("Line {}: no '=' found (malformed: {:?})", lineno + 1, trimmed));
            }
        }
    }

    (config, errors)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_path = if args.len() > 1 {
        &args[1]
    } else {
        "config.txt"
    };

    println!("Reading config from: {}", config_path);
    let (config, errors) = parse_config(config_path);

    // Report errors/warnings
    for err in &errors {
        eprintln!("{}", err);
    }

    // Print parsed config
    if config.is_empty() {
        println!("\nNo configuration loaded.");
    } else {
        println!("\nParsed {} key-value pairs:", config.len());
        let mut keys: Vec<&String> = config.keys().collect();
        keys.sort();
        for key in keys {
            println!("  {} = {}", key, config.get(key).unwrap());
        }
    }
}
