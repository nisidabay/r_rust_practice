// ex04_validate_csv — Validate CSV rows, collect all errors before reporting
//
// BONUS exercise: Write a validator for a simple CSV format.
// Each line: `name,age,email`
// Rules:
//   - name: must not be empty
//   - age: must be a valid u8 (0-255)
//   - email: must contain '@'
//
// Instead of failing on the first error, collect ALL errors across all lines
// and return them as a list. This is "validation" (all errors at once)
// vs "parsing" (fail-fast).
//
// Define your own error structs/enums.

use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

/// Custom error type for CSV validation, covering all failure modes.
#[derive(Debug)]
enum CsvError {
    WrongColumns(usize, usize),       // (line, actual column count)
    EmptyName(usize),                 // (line)
    InvalidAge(usize, String),        // (line, value)
    InvalidEmail(usize, String),      // (line, email)
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsvError::WrongColumns(line, count) => {
                write!(f, "line {line}: expected 3 columns, got {count}")
            }
            CsvError::EmptyName(line) => {
                write!(f, "line {line}: name is empty")
            }
            CsvError::InvalidAge(line, val) => {
                write!(f, "line {line}: invalid age \"{val}\"")
            }
            CsvError::InvalidEmail(line, email) => {
                write!(f, "line {line}: invalid email \"{email}\" (missing '@')")
            }
        }
    }
}

impl std::error::Error for CsvError {}

/// Validate a CSV string and return parsed Persons, or collect ALL errors.
fn validate_csv(content: &str) -> Result<Vec<Person>, Vec<CsvError>> {
    let mut people = Vec::new();
    let mut errors = Vec::new();

    for (i, line) in content.lines().enumerate() {
        let line_num = i + 1;

        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        let cols: Vec<&str> = line.split(',').collect();

        if cols.len() != 3 {
            errors.push(CsvError::WrongColumns(line_num, cols.len()));
            continue;
        }

        let name = cols[0].trim();
        let age_str = cols[1].trim();
        let email = cols[2].trim();

        // Validate name
        if name.is_empty() {
            errors.push(CsvError::EmptyName(line_num));
        }

        // Validate age
        let parsed_age = match age_str.parse::<u8>() {
            Ok(a) => Some(a),
            Err(_) => {
                errors.push(CsvError::InvalidAge(line_num, age_str.to_string()));
                None
            }
        };

        // Validate email
        if !email.contains('@') {
            errors.push(CsvError::InvalidEmail(line_num, email.to_string()));
        }

        // Only push if no errors on this line
        if !name.is_empty() && parsed_age.is_some() && email.contains('@') {
            people.push(Person {
                name: name.to_string(),
                age: parsed_age.unwrap(),
                email: email.to_string(),
            });
        }
    }

    if errors.is_empty() {
        Ok(people)
    } else {
        Err(errors)
    }
}

fn main() {
    let good_csv = "\
Alice,30,alice@example.com
Bob,25,bob@test.org
Carol,40,carol@work.com
";

    let bad_csv = "\
,30,alice@example.com
Bob,two-hundred,fake-email
Dave,20,dave@
Eve,25,
";

    // Test good CSV
    println!("=== Good CSV ===");
    match validate_csv(good_csv) {
        Ok(people) => {
            for p in &people {
                println!("  {:?}", p);
            }
        }
        Err(errors) => {
            for e in &errors {
                println!("  Error: {e}");
            }
        }
    }

    // Test bad CSV — should collect all errors
    println!();
    println!("=== Bad CSV ===");
    match validate_csv(bad_csv) {
        Ok(_) => println!("Unexpected success"),
        Err(errors) => {
            println!("Found {} error(s):", errors.len());
            for e in &errors {
                println!("  - {e}");
            }
        }
    }

    // Test empty CSV
    println!();
    println!("=== Empty CSV ===");
    match validate_csv("") {
        Ok(people) => println!("{} person(s) parsed", people.len()),
        Err(errors) => {
            for e in &errors {
                println!("  Error: {e}");
            }
        }
    }
}
