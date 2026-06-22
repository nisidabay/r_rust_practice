// ex01_grep.rs
// grep clone — Read stdin, print lines matching a pattern
//
// Flags:
//   -i : case-insensitive matching
//   -n : print line numbers
//   -c : print count of matching lines
//
// Usage:
//   echo -e "hello world\ngoodbye\nHello again" | rustc ex01_grep.rs && ./ex01_grep hello -i -n -c
//   cat somefile.txt | ./ex01_grep pattern -i

use std::env;
use std::io::{self, BufRead};

struct GrepConfig {
    pattern: String,
    ignore_case: bool,
    line_numbers: bool,
    count_only: bool,
}

fn parse_args() -> Result<GrepConfig, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: ex01_grep <pattern> [flags]".to_string());
    }

    let pattern = args[1].clone();
    let mut ignore_case = false;
    let mut line_numbers = false;
    let mut count_only = false;

    for arg in &args[2..] {
        match arg.as_str() {
            "-i" => ignore_case = true,
            "-n" => line_numbers = true,
            "-c" => count_only = true,
            f => return Err(format!("Unknown flag: {}", f)),
        }
    }

    Ok(GrepConfig {
        pattern,
        ignore_case,
        line_numbers,
        count_only,
    })
}

fn main() -> Result<(), String> {
    let config = parse_args()?;

    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut match_count = 0;
    let mut output_lines: Vec<String> = Vec::new();

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result.map_err(|e| format!("Read error: {}", e))?;

        let matches = if config.ignore_case {
            line.to_lowercase().contains(&config.pattern.to_lowercase())
        } else {
            line.contains(&config.pattern)
        };

        if matches {
            match_count += 1;
            if !config.count_only {
                let prefix = if config.line_numbers {
                    format!("{}:", line_num + 1)
                } else {
                    String::new()
                };
                output_lines.push(format!("{}{}", prefix, line));
            }
        }
    }

    if config.count_only {
        println!("{}", match_count);
    } else {
        for line in output_lines {
            println!("{}", line);
        }
    }

    Ok(())
}
