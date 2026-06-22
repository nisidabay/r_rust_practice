// Project: pretty printer
//
// Read structured data in format "name: Alice, age: 30, scores: [85,92,78]"
// Parse and pretty-print with ANSI colors and alignment.
// Support --json flag. Use traits for different output formats.
//
// Usage:
//   cargo run -- "name: Alice, age: 30, scores: [85,92,78]"
//   cargo run -- "name: Alice, age: 30, scores: [85,92,78]" --json
//   echo "name: Bob, age: 25, scores: [90,88]" | cargo run --

use std::env;
use std::io::{self, BufRead};

// --- Data model ---

#[derive(Debug, Clone)]
struct Record {
    name: String,
    age: u32,
    scores: Vec<u32>,
}

#[derive(Debug)]
enum ParseError {
    MissingField(String),
    InvalidAge(String),
    InvalidScores(String),
    InvalidFormat(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::MissingField(s) => write!(f, "missing field: {}", s),
            ParseError::InvalidAge(s) => write!(f, "invalid age: '{}'", s),
            ParseError::InvalidScores(s) => write!(f, "invalid scores: '{}'", s),
            ParseError::InvalidFormat(s) => write!(f, "invalid format: {}", s),
        }
    }
}

// --- Output format trait ---

trait FormatOutput {
    fn render(&self, records: &[Record]) -> String;
}

// --- Plain text output ---

struct PlainOutput;

impl FormatOutput for PlainOutput {
    fn render(&self, records: &[Record]) -> String {
        if records.is_empty() {
            return "No records.".to_string();
        }

        let mut output = String::new();
        output.push_str(&format!("\n  {:<12} {:<6} {}\n", "Name", "Age", "Scores"));
        output.push_str(&format!("  {}\n", "-".repeat(40)));

        for r in records {
            let scores_str = r
                .scores
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            output.push_str(&format!("  {:<12} {:<6} [{}]\n", r.name, r.age, scores_str));
        }
        output
    }
}

// --- Colored ANSI output ---

struct ColoredOutput;

impl FormatOutput for ColoredOutput {
    fn render(&self, records: &[Record]) -> String {
        if records.is_empty() {
            return "\x1b[33mNo records.\x1b[0m".to_string();
        }

        let mut output = String::new();
        output.push_str(&format!(
            "\n  \x1b[1;36m{:<12} {:<6} {}\x1b[0m\n",
            "Name", "Age", "Scores"
        ));
        output.push_str(&format!("  \x1b[2;37m{}\x1b[0m\n", "-".repeat(40)));

        for r in records {
            let scores_str = r
                .scores
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            output.push_str(&format!(
                "  \x1b[1;33m{:<12}\x1b[0m \x1b[1;32m{:<6}\x1b[0m \x1b[1;34m[{}]\x1b[0m\n",
                r.name, r.age, scores_str
            ));
        }
        output
    }
}

// --- JSON output ---

struct JsonOutput;

impl FormatOutput for JsonOutput {
    fn render(&self, records: &[Record]) -> String {
        if records.is_empty() {
            return "[]".to_string();
        }

        let mut output = String::from("[\n");
        for (i, r) in records.iter().enumerate() {
            let scores_str = r
                .scores
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            output.push_str(&format!(
                "  {{\n    \"name\": \"{}\",\n    \"age\": {},\n    \"scores\": [{}]\n  }}",
                r.name, r.age, scores_str
            ));
            if i < records.len() - 1 {
                output.push(',');
            }
            output.push('\n');
        }
        output.push(']');
        output
    }
}

// --- Parser ---

fn parse_record(line: &str) -> Result<Record, ParseError> {
    let line = line.trim();
    if line.is_empty() {
        return Err(ParseError::InvalidFormat("empty line".to_string()));
    }

    let mut name = String::new();
    let mut age: Option<u32> = None;
    let mut scores: Option<Vec<u32>> = None;

    // Parse fields more carefully to handle commas inside brackets
    for field in split_fields(line) {
        let (key, value) = field.split_once(':').ok_or_else(|| {
            ParseError::InvalidFormat(format!("no colon in '{}'", field))
        })?;
        let key = key.trim().to_lowercase();
        let value = value.trim();

        match key.as_str() {
            "name" => name = value.to_string(),
            "age" => {
                age = Some(
                    value
                        .parse()
                        .map_err(|_| ParseError::InvalidAge(value.to_string()))?,
                );
            }
            "scores" => {
                let inner = value
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .trim();
                if inner.is_empty() {
                    scores = Some(Vec::new());
                } else {
                    scores = Some(
                        inner
                            .split(',')
                            .map(|s| {
                                s.trim()
                                    .parse()
                                    .map_err(|_| ParseError::InvalidScores(s.trim().to_string()))
                            })
                            .collect::<Result<Vec<_>, _>>()?,
                    );
                }
            }
            _ => {
                return Err(ParseError::InvalidFormat(format!("unknown key '{}'", key)));
            }
        }
    }

    if name.is_empty() {
        return Err(ParseError::MissingField("name".to_string()));
    }
    let age = age.ok_or_else(|| ParseError::MissingField("age".to_string()))?;
    let scores = scores.unwrap_or_default();

    Ok(Record { name, age, scores })
}

/// Split a CSV-like line into fields, respecting brackets (so commas inside [...] don't split)
fn split_fields(line: &str) -> Vec<&str> {
    let mut fields = Vec::new();
    let mut depth = 0u32;
    let mut start = 0usize;

    for (i, c) in line.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                fields.push(line[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    if start < line.len() {
        fields.push(line[start..].trim());
    }
    fields
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let use_json = args.iter().any(|a| a == "--json");

    let records: Vec<Record> = if args.len() > 1 && !args[1].starts_with("--") {
        vec![parse_record(&args[1]).map_err(|e| e.to_string())?]
    } else {
        let stdin = io::stdin();
        let mut records = Vec::new();
        for line in stdin.lock().lines() {
            let line = line.map_err(|e| e.to_string())?;
            if line.trim().is_empty() {
                continue;
            }
            records.push(parse_record(&line).map_err(|e| e.to_string())?);
        }
        records
    };

    let output: Box<dyn FormatOutput> = if use_json {
        Box::new(JsonOutput)
    } else {
        Box::new(ColoredOutput)
    };

    print!("{}", output.render(&records));
    Ok(())
}
