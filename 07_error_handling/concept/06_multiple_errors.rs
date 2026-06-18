// 06_multiple_errors — Combining different error types, Box<dyn Error>
//
// Real programs call many fallible operations, each with its own error type.
// The ? operator needs a single unified error type. Common strategies:
//   1. Box<dyn std::error::Error>
//   2. Custom enum with From impls
//   3. thiserror crate (production)

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

// --- 1. Using Box<dyn Error> — simplest approach ---
// Any type that implements Display + Debug + Error can be converted
// into Box<dyn Error> automatically via ?.

fn read_and_parse_boxed(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // io::Error -> Box<dyn Error> (auto-converted by ?)
    let content = fs::read_to_string(path)?;
    // ParseIntError -> Box<dyn Error>
    let num: i32 = content.trim().parse()?;
    Ok(num)
}

// --- 2. Custom error enum wrapping multiple error types ---
// This gives callers full type information.

#[derive(Debug)]
enum ConfigError {
    Io(io::Error),
    Parse(ParseIntError),
    EmptyFile,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "I/O error: {e}"),
            ConfigError::Parse(e) => write!(f, "parse error: {e}"),
            ConfigError::EmptyFile => write!(f, "file is empty"),
        }
    }
}

impl std::error::Error for ConfigError {
    // Provide source() so callers can inspect the underlying error
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(e) => Some(e),
            ConfigError::Parse(e) => Some(e),
            ConfigError::EmptyFile => None,
        }
    }
}

// --- 3. From impls let ? do the conversion ---
impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::Parse(e)
    }
}

// --- 4. Now ? works with ConfigError ---
fn read_config(path: &str) -> Result<i32, ConfigError> {
    let content = fs::read_to_string(path)?; // io::Error -> ConfigError
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Err(ConfigError::EmptyFile);
    }
    let num: i32 = trimmed.parse()?; // ParseIntError -> ConfigError
    Ok(num)
}

fn main() {
    // --- 5. Work with the custom error ---
    match read_config("/tmp/nonexistent_config.txt") {
        Ok(val) => println!("Config value: {val}"),
        Err(ConfigError::Io(e)) => {
            println!("I/O error (we can match specifically): {e}");
        }
        Err(e) => {
            println!("Other config error: {e}");
            // Check the source:
            if let Some(source) = e.source() {
                println!("  Caused by: {source}");
            }
        }
    }

    // --- 6. Or use Box<dyn Error> for simplicity ---
    match read_and_parse_boxed("/tmp/nonexistent.txt") {
        Ok(val) => println!("Boxed result: {val}"),
        Err(e) => println!("Boxed error: {e}"),
    }

    // --- 7. Combining multiple Results with map_err ---
    // map_err converts error types manually (good for one-offs):
    fn explicit_convert(path: &str) -> Result<i32, String> {
        let content = fs::read_to_string(path).map_err(|e| format!("IO: {e}"))?;
        let num: i32 = content.trim().parse().map_err(|e| format!("Parse: {e}"))?;
        Ok(num)
    }

    match explicit_convert("/tmp/nonexistent.txt") {
        Ok(v) => println!("Explicit: {v}"),
        Err(e) => println!("Explicit error: {e}"),
    }

    println!("--- 06_multiple_errors done ---");
}
