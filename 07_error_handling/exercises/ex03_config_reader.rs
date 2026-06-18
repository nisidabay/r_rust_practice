// ex03_config_reader — Read config from file, parse key=value, report errors
//
// Problem: Write a function `read_config(path: &str) -> Result<Vec<(usize, String, String)>, String>`
// that reads a config file where each line is `key=value`.
//
// Return Ok(list of (line_number, key, value) tuples) on success.
// On error, return a descriptive string like:
//   "line 3: missing '=' delimiter"
//   "line 5: empty key"
//   "error reading file: No such file or directory (os error 2)"
//
// Lines starting with '#' are comments and should be skipped.
// Empty lines should be skipped.
// Keys must not be empty (e.g. "=value" is invalid).
// Values can be empty (e.g. "key=" is fine).

use std::fs;

fn read_config(path: &str) -> Result<Vec<(usize, String, String)>, String> {
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("error reading file: {e}"))?;

    let mut entries = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        let line_num = i + 1; // 1-based line numbers
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Split on the first '=' only
        let parts: Vec<&str> = trimmed.splitn(2, '=').collect();

        if parts.len() < 2 {
            return Err(format!("line {line_num}: missing '=' delimiter"));
        }

        let key = parts[0].trim().to_string();
        let value = parts[1].trim().to_string();

        if key.is_empty() {
            return Err(format!("line {line_num}: empty key"));
        }

        entries.push((line_num, key, value));
    }

    Ok(entries)
}

fn main() {
    // Create a test config
    let test_config = "\
# Server config
host=localhost
port=8080

debug=true
=bad_key
name=Hercules
trailing==equals
";

    let test_path = "/tmp/test_config.txt";
    fs::write(test_path, test_config).expect("write test config");

    match read_config(test_path) {
        Ok(entries) => {
            println!("Config entries ({}):", entries.len());
            for (line, key, value) in &entries {
                println!("  line {line}: \"{key}\" = \"{value}\"");
            }
            // Clean up
            let _ = fs::remove_file(test_path);
        }
        Err(e) => println!("Error: {e}"),
    }

    // Test with missing file
    match read_config("/tmp/nonexistent_config_xyz.txt") {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Expected error: {e}"),
    }
}
