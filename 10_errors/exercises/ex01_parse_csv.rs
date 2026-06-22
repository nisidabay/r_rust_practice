// ex01_parse_csv.rs — Read CSV line, parse fields. Return Result for malformed lines.
// Print errors to stderr.

use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut line_count = 0u64;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        line_count += 1;

        // Parse: each line should have exactly 3 comma-separated fields
        // Expected format: "name,age,city"
        let result = parse_csv_line(&line);
        match result {
            Ok(fields) => {
                println!("Line {}: {} | {} | {}", line_count, fields[0], fields[1], fields[2]);
            }
            Err(e) => {
                writeln!(io::stderr(), "Error line {}: {}", line_count, e).unwrap();
            }
        }
    }
}

fn parse_csv_line(line: &str) -> Result<Vec<String>, String> {
    if line.trim().is_empty() {
        return Err("empty line".to_string());
    }
    let fields: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
    if fields.len() != 3 {
        return Err(format!("expected 3 fields, got {}", fields.len()));
    }
    Ok(fields)
}
