// ex03_config_reader.rs — Read KEY=VALUE lines. Return Result with custom ConfigError enum
// Handles: MissingKey, ParseError, DuplicateKey

use std::collections::HashMap;
use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug)]
enum ConfigError {
    MissingKey,
    ParseError(String),
    DuplicateKey(String),
    IoError(std::io::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingKey => write!(f, "line has no key (format: KEY=VALUE)"),
            ConfigError::ParseError(s) => write!(f, "parse error: '{}' is not valid", s),
            ConfigError::DuplicateKey(k) => write!(f, "duplicate key: '{}'", k),
            ConfigError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::IoError(e)
    }
}

fn parse_config_line(line: &str) -> Result<(String, String), ConfigError> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        // Skip empty lines and comments
        return Err(ConfigError::MissingKey);
    }
    let (key, value) = line.split_once('=').ok_or(ConfigError::MissingKey)?;
    let key = key.trim().to_string();
    if key.is_empty() {
        return Err(ConfigError::MissingKey);
    }
    let value = value.trim().to_string();
    Ok((key, value))
}

fn main() {
    let stdin = io::stdin();
    let mut config: HashMap<String, String> = HashMap::new();
    let mut line_num = 0u64;

    for line in stdin.lock().lines() {
        let line = line.unwrap(); // propagate I/O error via From
        line_num += 1;

        match parse_config_line(&line) {
            Ok((key, value)) => {
                // Check for duplicates
                if config.contains_key(&key) {
                    eprintln!("Warning line {}: {}", line_num, ConfigError::DuplicateKey(key.clone()));
                }
                config.insert(key, value);
            }
            Err(ConfigError::MissingKey) => {
                // Skip comments/empty lines silently
            }
            Err(e) => {
                eprintln!("Error line {}: {}", line_num, e);
            }
        }
    }

    println!("Parsed {} config entries:", config.len());
    let mut keys: Vec<&String> = config.keys().collect();
    keys.sort();
    for key in keys {
        println!("  {} = {}", key, config[key]);
    }
}
