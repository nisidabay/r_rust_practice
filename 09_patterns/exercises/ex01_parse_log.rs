// ex01_parse_log.rs — Parse log lines "ERROR: disk full", "INFO: starting"
// Match on the log level, display the message

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim();

        // Split on ": " — first part is level, rest is message
        match line.split_once(": ") {
            Some((level, message)) => {
                // Match on the level string
                match level {
                    "ERROR" | "FATAL" => {
                        println!("\x1b[31m[ERROR]\x1b[0m {}", message);
                    }
                    "WARN" | "WARNING" => {
                        println!("\x1b[33m[WARN]\x1b[0m  {}", message);
                    }
                    "INFO" => {
                        println!("\x1b[32m[INFO]\x1b[0m  {}", message);
                    }
                    "DEBUG" | "TRACE" => {
                        println!("\x1b[90m[DEBUG]\x1b[0m {}", message);
                    }
                    _ => {
                        println!("[{}] {}", level, message);
                    }
                }
            }
            None => {
                // No colon separator — just echo it raw
                println!("{}", line);
            }
        }
    }
}
