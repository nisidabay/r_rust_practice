// 03_result_chain.rs
// Capstone — Chaining Results with ? (Read → Parse → Validate → Write)
//
// Concepts: Result, ?, custom errors, pipeline pattern
//
// Usage:
//   rustc 03_result_chain.rs && ./03_result_chain

use std::fs;
use std::io;
use std::num::ParseIntError;
use std::path::Path;
use std::fmt;

// --- Custom error type ---

#[derive(Debug)]
enum PipelineError {
    ReadError(io::Error),
    ParseError(String),
    ValidateError(String),
    WriteError(io::Error),
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineError::ReadError(e) => write!(f, "Read failed: {}", e),
            PipelineError::ParseError(msg) => write!(f, "Parse failed: {}", msg),
            PipelineError::ValidateError(msg) => write!(f, "Validation failed: {}", msg),
            PipelineError::WriteError(e) => write!(f, "Write failed: {}", e),
        }
    }
}

impl std::error::Error for PipelineError {}

impl From<io::Error> for PipelineError {
    fn from(e: io::Error) -> Self {
        PipelineError::ReadError(e)
    }
}

impl From<ParseIntError> for PipelineError {
    fn from(_e: ParseIntError) -> Self {
        PipelineError::ParseError("invalid integer".to_string())
    }
}

// --- Data types ---

#[derive(Debug)]
struct Record {
    name: String,
    age: u32,
    score: u32,
}

// --- Pipeline steps ---

/// Step 1: Read raw text from a file
fn read_input(path: &str) -> Result<String, PipelineError> {
    println!("[1/4] Reading input from '{}'...", path);
    let content = fs::read_to_string(path)?;
    println!("       Read {} bytes", content.len());
    Ok(content)
}

/// Step 2: Parse structured records from CSV-like text
fn parse_records(content: &str) -> Result<Vec<Record>, PipelineError> {
    println!("[2/4] Parsing records...");
    let mut records = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            return Err(PipelineError::ParseError(format!(
                "line {}: expected 3 fields, got {}",
                line_num + 1,
                parts.len()
            )));
        }
        let record = Record {
            name: parts[0].trim().to_string(),
            age: parts[1].trim().parse()?,
            score: parts[2].trim().parse()?,
        };
        records.push(record);
    }

    println!("       Parsed {} records", records.len());
    Ok(records)
}

/// Step 3: Validate records
fn validate_records(records: &[Record]) -> Result<(), PipelineError> {
    println!("[3/4] Validating records...");
    for (i, record) in records.iter().enumerate() {
        if record.name.is_empty() {
            return Err(PipelineError::ValidateError(format!(
                "record {}: name is empty",
                i + 1
            )));
        }
        if record.age > 150 {
            return Err(PipelineError::ValidateError(format!(
                "record {}: age {} is unrealistic",
                i + 1,
                record.age
            )));
        }
        if record.score > 100 {
            return Err(PipelineError::ValidateError(format!(
                "record {}: score {} exceeds maximum 100",
                i + 1,
                record.score
            )));
        }
    }
    println!("       All {} records valid", records.len());
    Ok(())
}

/// Step 4: Write summary report
fn write_report(records: &[Record], path: &str) -> Result<(), PipelineError> {
    println!("[4/4] Writing report to '{}'...", path);

    let mut report = String::new();
    report.push_str("=== Record Summary ===\n");
    report.push_str(&format!("Total records: {}\n", records.len()));
    report.push_str(&format!(
        "Average age: {:.1}\n",
        records.iter().map(|r| r.age).sum::<u32>() as f64 / records.len() as f64
    ));
    report.push_str(&format!(
        "Average score: {:.1}\n",
        records.iter().map(|r| r.score).sum::<u32>() as f64 / records.len() as f64
    ));
    report.push_str("\n--- Details ---\n");
    for record in records {
        report.push_str(&format!(
            "  {:20} | Age: {:3} | Score: {:3}\n",
            record.name, record.age, record.score
        ));
    }

    // This From<io::Error> converts to PipelineError::WriteError
    fs::write(path, &report)?;
    println!("       Wrote {} bytes", report.len());
    Ok(())
}

fn main() -> Result<(), PipelineError> {
    let input_path = "records_input.txt";
    let output_path = "records_report.txt";

    // Create sample input if it doesn't exist
    if !Path::new(input_path).exists() {
        fs::write(
            input_path,
            "Alice,30,95\nBob,25,88\nCharlie,35,92\n# This is a comment\nDiana,28,78\n",
        )
        .map_err(PipelineError::WriteError)?;
        println!("Created sample input '{}'", input_path);
    }

    // --- Pipeline: Read → Parse → Validate → Write ---
    let content = read_input(input_path)?;
    let records = parse_records(&content)?;
    validate_records(&records)?;
    write_report(&records, output_path)?;

    println!();
    println!("=== Pipeline complete! ===");
    println!("Report written to '{}'", output_path);

    // Show the report
    let report = fs::read_to_string(output_path).unwrap();
    println!();
    println!("{}", report);

    // Clean up
    fs::remove_file(input_path)?;
    fs::remove_file(output_path)?;
    println!("Cleaned up temporary files");

    Ok(())
}
