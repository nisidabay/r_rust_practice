// log_analyzer — CLI log file analyzer using iterator chains
//
// Reads a log file line by line, parses structured log entries,
// and supports filtering by level and showing top N entries.
//
// Usage:
//   cargo run -- path/to/log --level ERROR --top 10
//   cargo run -- path/to/log --help

use std::env;
use std::fs;
use std::process;

/// A parsed log entry.
#[derive(Debug, Clone)]
struct LogEntry {
    /// Timestamp string (e.g. "2025-06-18 10:30:45")
    timestamp: String,
    /// Log level: INFO, WARN, ERROR, DEBUG, TRACE
    level: String,
    /// The log message
    message: String,
}

/// Parse a single log line into a LogEntry.
/// Expected format: "2025-06-18 10:30:45 [LEVEL] Message here"
fn parse_log_line(line: &str) -> Option<LogEntry> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // Split at the first '[' to get level
    let bracket_start = line.find('[')?;
    let bracket_end = line.find(']')?;

    let timestamp = line[..bracket_start].trim().to_string();
    let level = line[bracket_start + 1..bracket_end].trim().to_string();
    let message = line[bracket_end + 1..].trim().to_string();

    Some(LogEntry {
        timestamp,
        level,
        message,
    })
}

/// Filter entries by the given log level(s).
fn filter_by_level<'a>(
    entries: impl Iterator<Item = &'a LogEntry> + 'a,
    level_filter: &'a str,
) -> impl Iterator<Item = &'a LogEntry> + 'a {
    let levels: Vec<&str> = level_filter.split(',').map(|s| s.trim()).collect();
    entries.filter(move |e| levels.contains(&e.level.as_str()))
}

/// Print a help message.
fn print_help() {
    eprintln!("Usage: log_analyzer <file> [options]");
    eprintln!();
    eprintln!("Reads a log file and displays statistics and filtered entries.");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  <file>             Path to the log file");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --level <LEVEL>    Filter by log level (e.g. ERROR, WARN, INFO)");
    eprintln!("                     Multiple levels: --level ERROR,WARN");
    eprintln!("  --top <N>          Show top N entries (default: all)");
    eprintln!("  --help             Show this help message");
    eprintln!();
    eprintln!("Log format expected:");
    eprintln!("  2025-06-18 10:30:45 [LEVEL] Message here");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse arguments
    let mut file_path: Option<String> = None;
    let mut level_filter: Option<String> = None;
    let mut top_n: Option<usize> = None;
    let mut show_help = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" => {
                show_help = true;
            }
            "--level" => {
                i += 1;
                if i < args.len() {
                    level_filter = Some(args[i].clone());
                }
            }
            "--top" => {
                i += 1;
                if i < args.len() {
                    top_n = args[i].parse().ok();
                }
            }
            s if !s.starts_with("--") => {
                file_path = Some(args[i].clone());
            }
            _ => {}
        }
        i += 1;
    }

    if show_help || file_path.is_none() {
        print_help();
        process::exit(if file_path.is_none() { 1 } else { 0 });
    }

    let path = file_path.unwrap();
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            process::exit(1);
        }
    };

    // Parse all lines into LogEntry using iterator chain
    let entries: Vec<LogEntry> = content
        .lines()
        .filter_map(parse_log_line)
        .collect();

    if entries.is_empty() {
        println!("No log entries found in '{}'", path);
        process::exit(0);
    }

    // --- Statistics ---
    println!("=== Log Summary ===");
    println!("Total entries: {}", entries.len());

    // Count by level using iterator chain
    let level_counts: Vec<(String, usize)> = entries
        .iter()
        .map(|e| e.level.clone())
        .fold(
            std::collections::HashMap::new(),
            |mut acc, level| {
                *acc.entry(level).or_insert(0) += 1;
                acc
            },
        )
        .into_iter()
        .collect();

    println!("\nCount by level:");
    for (level, count) in &level_counts {
        println!("  {level:<8} {count}");
    }

    // --- Filtered output ---
    let filtered: Box<dyn Iterator<Item = &LogEntry>> = match &level_filter {
        Some(lf) => Box::new(filter_by_level(entries.iter(), lf)),
        None => Box::new(entries.iter()),
    };

    let top_entries: Vec<&LogEntry> = match top_n {
        Some(n) => filtered.take(n).collect(),
        None => filtered.collect(),
    };

    let level_label = level_filter.as_deref().unwrap_or("ALL");
    println!("\n=== Log Entries ({level_label}) ===");
    if let Some(n) = top_n {
        println!("Showing top {n} of {} matching entries", entries.len());
    }

    for entry in &top_entries {
        println!("[{}] [{}] {}", entry.timestamp, entry.level, entry.message);
    }

    println!("\nAnalysis complete.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_line_valid() {
        let line = "2025-06-18 10:30:45 [ERROR] Something went wrong";
        let entry = parse_log_line(line).unwrap();
        assert_eq!(entry.timestamp, "2025-06-18 10:30:45");
        assert_eq!(entry.level, "ERROR");
        assert_eq!(entry.message, "Something went wrong");
    }

    #[test]
    fn test_parse_log_line_empty() {
        assert!(parse_log_line("").is_none());
        assert!(parse_log_line("   ").is_none());
    }

    #[test]
    fn test_parse_log_line_missing_bracket() {
        assert!(parse_log_line("no brackets here").is_none());
    }

    #[test]
    fn test_filter_by_level() {
        let entries = vec![
            LogEntry {
                timestamp: "a".into(),
                level: "ERROR".into(),
                message: "err1".into(),
            },
            LogEntry {
                timestamp: "b".into(),
                level: "WARN".into(),
                message: "warn1".into(),
            },
            LogEntry {
                timestamp: "c".into(),
                level: "ERROR".into(),
                message: "err2".into(),
            },
            LogEntry {
                timestamp: "d".into(),
                level: "INFO".into(),
                message: "info1".into(),
            },
        ];

        let errors: Vec<&LogEntry> = filter_by_level(entries.iter(), "ERROR").collect();
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].message, "err1");
        assert_eq!(errors[1].message, "err2");

        let multi: Vec<&LogEntry> = filter_by_level(entries.iter(), "ERROR,WARN").collect();
        assert_eq!(multi.len(), 3);
    }

    #[test]
    fn test_iterator_chain_composition() {
        // Simulate a full pipeline using only iterator chains
        let log_lines = vec![
            "2025-01-01 [INFO] Startup",
            "2025-01-01 [ERROR] Crash",
            "2025-01-02 [WARN] Degraded",
            "2025-01-02 [ERROR] Timeout",
            "",
            "2025-01-03 [INFO] Recovery",
        ];

        let result: Vec<String> = log_lines
            .iter()
            .filter_map(|line| parse_log_line(line))
            .filter(|e| e.level == "ERROR")
            .map(|e| format!("{}: {}", e.timestamp, e.message))
            .collect();

        assert_eq!(result.len(), 2);
        assert!(result[0].contains("Crash"));
        assert!(result[1].contains("Timeout"));
    }
}
