// Project: Log line counter
//
// Read a log file, count INFO/WARN/ERROR lines.
// Output a summary table.
// --level ERROR to filter one level
// --top N to show top N lines by level
// Handle missing file with custom error
//
// Usage:
//   cargo run -- path/to/logfile
//   cargo run -- path/to/logfile --level ERROR --top 5
//   cargo run -- nonexistent.log

use std::collections::BTreeMap;
use std::env;
use std::fmt;
use std::fs;
use std::io::{BufRead, BufReader};

// --- Custom error type ---

#[derive(Debug)]
enum LogError {
    FileNotFound(String),
    IoError(std::io::Error),
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogError::FileNotFound(path) => write!(f, "Error: file not found — '{}'", path),
            LogError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for LogError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LogError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for LogError {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::NotFound => {
                LogError::FileNotFound("(path provided)".to_string())
            }
            _ => LogError::IoError(e),
        }
    }
}

// --- Config ---

struct Config {
    path: String,
    level: Option<String>,
    top: Option<usize>,
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: log_counter <file> [--level LEVEL] [--top N]".to_string());
    }

    let mut path = String::new();
    let mut level: Option<String> = None;
    let mut top: Option<usize> = None;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--level" => {
                i += 1;
                if i < args.len() {
                    level = Some(args[i].to_uppercase());
                } else {
                    return Err("--level requires a value (e.g. ERROR)".to_string());
                }
            }
            "--top" => {
                i += 1;
                if i < args.len() {
                    top = Some(
                        args[i]
                            .parse()
                            .map_err(|_| format!("invalid --top value: '{}'", args[i]))?,
                    );
                } else {
                    return Err("--top requires a number".to_string());
                }
            }
            s if s.starts_with('-') => {
                return Err(format!("unknown flag: {}", s));
            }
            _ => {
                if path.is_empty() {
                    path = args[i].clone();
                } else {
                    return Err(format!("unexpected argument: {}", args[i]));
                }
            }
        }
        i += 1;
    }

    if path.is_empty() {
        return Err("no file specified".to_string());
    }

    Ok(Config { path, level, top })
}

// --- Log parsing ---

#[derive(Debug)]
struct LogEntry {
    line_num: usize,
    content: String,
}

struct LogStats {
    total: usize,
    by_level: BTreeMap<String, usize>,
    level_lines: BTreeMap<String, Vec<LogEntry>>,
}

fn analyze_log(path: &str, filter_level: Option<&str>) -> Result<LogStats, LogError> {
    let file_path = std::path::Path::new(path);
    if !file_path.exists() {
        return Err(LogError::FileNotFound(path.to_string()));
    }

    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut stats = LogStats {
        total: 0,
        by_level: BTreeMap::new(),
        level_lines: BTreeMap::new(),
    };

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        stats.total += 1;

        // Determine log level: look for INFO, WARN, ERROR (case-insensitive)
        let upper = line.to_uppercase();
        let level = if upper.contains("ERROR") {
            "ERROR"
        } else if upper.contains("WARN") || upper.contains("WARNING") {
            "WARN"
        } else if upper.contains("INFO") {
            "INFO"
        } else {
            "OTHER"
        };

        // Apply level filter
        if let Some(filter) = filter_level {
            if level != filter {
                continue;
            }
        }

        *stats.by_level.entry(level.to_string()).or_insert(0) += 1;
        stats
            .level_lines
            .entry(level.to_string())
            .or_default()
            .push(LogEntry {
                line_num: line_num + 1,
                content: line,
            });
    }

    Ok(stats)
}

fn print_summary(stats: &LogStats, top: Option<usize>) {
    println!("\n╔══════════════════════════════════╗");
    println!("║       Log Summary Report        ║");
    println!("╚══════════════════════════════════╝");
    println!();
    println!("  Total lines read: {}", stats.total);
    println!();

    if stats.by_level.is_empty() {
        println!("  No matching log lines found.");
        return;
    }

    // Summary table
    println!("  {:<8} {:>8} {:>8}", "Level", "Count", "%");
    println!("  {}", "-".repeat(30));

    for (level, count) in &stats.by_level {
        let pct = if stats.total > 0 {
            (*count as f64 / stats.total as f64) * 100.0
        } else {
            0.0
        };
        let color = match level.as_str() {
            "ERROR" => "\x1b[1;31m",
            "WARN" => "\x1b[1;33m",
            "INFO" => "\x1b[1;32m",
            _ => "\x1b[2;37m",
        };
        println!(
            "  {}{:<8}\x1b[0m {:>8} {:>7.1}%",
            color, level, count, pct
        );
    }

    // Top N lines
    if let Some(n) = top {
        if n > 0 {
            println!();
            println!("  Top {} lines by level:", n);

            let mut level_order: Vec<&String> = stats.by_level.keys().collect();
            // Sort by priority: ERROR first, then WARN, then INFO, then OTHER
            level_order.sort_by_key(|k| match k.as_str() {
                "ERROR" => 0,
                "WARN" => 1,
                "INFO" => 2,
                _ => 3,
            });

            for level in level_order {
                if let Some(lines) = stats.level_lines.get(level) {
                    let color = match level.as_str() {
                        "ERROR" => "\x1b[1;31m",
                        "WARN" => "\x1b[1;33m",
                        "INFO" => "\x1b[1;32m",
                        _ => "",
                    };
                    println!("\n  {}{}\x1b[0m:", color, level);
                    let show = lines.iter().take(n);
                    for entry in show {
                        let truncated = if entry.content.len() > 80 {
                            format!("{}...", &entry.content[..77])
                        } else {
                            entry.content.clone()
                        };
                        println!("    {:>6}: {}", entry.line_num, truncated);
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), LogError> {
    let config = match parse_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let stats = analyze_log(&config.path, config.level.as_deref())?;
    print_summary(&stats, config.top);
    Ok(())
}
