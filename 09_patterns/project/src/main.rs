// Project: mini grep — search stdin for a pattern, like grep but minimal
//
// Supports:
//   --ignore-case  (-i)  case-insensitive search
//   --count        (-c)  only show match count, not matching lines
//   --line-number  (-n)  prefix each match with line number
//
// Colors the matched portion in red using ANSI escape codes.
// Uses pattern matching (match, if let) to dispatch flags and format output.

use std::env;
use std::io::{self, BufRead};

struct Config {
    pattern: String,
    ignore_case: bool,
    count_only: bool,
    line_numbers: bool,
}

#[derive(Debug)]
enum Flag {
    IgnoreCase,
    CountOnly,
    LineNumber,
    Unknown(String),
}

fn parse_flag(arg: &str) -> Flag {
    match arg {
        "-i" | "--ignore-case" => Flag::IgnoreCase,
        "-c" | "--count" => Flag::CountOnly,
        "-n" | "--line-number" => Flag::LineNumber,
        s if s.starts_with("--") => {
            // Parse --key=value (though grep doesn't use these, we support it)
            if let Some((key, _val)) = s.split_once('=') {
                match key {
                    "--ignore-case" => Flag::IgnoreCase,
                    "--count" => Flag::CountOnly,
                    "--line-number" => Flag::LineNumber,
                    _ => Flag::Unknown(s.to_string()),
                }
            } else {
                Flag::Unknown(s.to_string())
            }
        }
        _ => Flag::Unknown(arg.to_string()),
    }
}

fn build_config(args: &[String]) -> Config {
    let mut pattern = String::new();
    let mut ignore_case = false;
    let mut count_only = false;
    let mut line_numbers = false;

    let mut i = 1;
    while i < args.len() {
        match parse_flag(&args[i]) {
            Flag::IgnoreCase => ignore_case = true,
            Flag::CountOnly => count_only = true,
            Flag::LineNumber => line_numbers = true,
            Flag::Unknown(_) => {
                // First non-flag argument is the pattern
                if pattern.is_empty() {
                    pattern = args[i].clone();
                }
            }
        }
        i += 1;
    }

    Config { pattern, ignore_case, count_only, line_numbers }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = build_config(&args);

    if cfg.pattern.is_empty() {
        eprintln!("Usage: mini_grep [--ignore-case] [--count] [--line-number] <pattern>");
        std::process::exit(1);
    }

    let stdin = io::stdin();
    let mut match_count = 0u64;
    let line_width = 4; // min-width for line numbers

    for (line_num, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();

        // Check for match (with or without case sensitivity)
        let matched = if cfg.ignore_case {
            line.to_lowercase().contains(&cfg.pattern.to_lowercase())
        } else {
            line.contains(&cfg.pattern)
        };

        if matched {
            match_count += 1;

            if cfg.count_only {
                // Count-only mode: just increment, don't print lines
                continue;
            }

            // Print line number prefix if enabled
            if cfg.line_numbers {
                print!("{:width$}: ", line_num + 1, width = line_width);
            }

            // Find and highlight the pattern in the line
            let search_line = if cfg.ignore_case {
                line.to_lowercase()
            } else {
                line.clone()
            };

            let pat = if cfg.ignore_case {
                cfg.pattern.to_lowercase()
            } else {
                cfg.pattern.clone()
            };

            // Find all occurrences and print with red highlighting
            if let Some(pos) = search_line.find(&pat) {
                // Print before match
                print!("{}", &line[..pos]);
                // Print match in red
                print!("\x1b[31m{}\x1b[0m", &line[pos..pos + pat.len()]);
                // Print after match
                println!("{}", &line[pos + pat.len()..]);
            } else {
                println!("{}", line);
            }
        }
    }

    // Print count if requested
    if cfg.count_only {
        println!("{}", match_count);
    }
}
