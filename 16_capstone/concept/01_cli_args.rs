// 01_cli_args.rs
// Capstone — Manual CLI flag parsing (no crates)
//
// Concepts: env::args(), pattern matching, "--key=value" and "--flag" parsing
//
// Usage:
//   rustc 01_cli_args.rs && ./01_cli_args --name=Alice --verbose --count=3
//   rustc 01_cli_args.rs && ./01_cli_args --help

use std::env;

#[derive(Debug)]
struct Args {
    name: Option<String>,
    verbose: bool,
    count: usize,
    help: bool,
}

fn parse_args() -> Args {
    let mut args = Args {
        name: None,
        verbose: false,
        count: 1,
        help: false,
    };

    for arg in env::args().skip(1) {
        // --flag or --key=value
        if arg == "--help" || arg == "-h" {
            args.help = true;
        } else if arg == "--verbose" || arg == "-v" {
            args.verbose = true;
        } else if let Some(key_value) = arg.strip_prefix("--") {
            if let Some(eq_pos) = key_value.find('=') {
                let key = &key_value[..eq_pos];
                let value = &key_value[eq_pos + 1..];
                match key {
                    "name" => args.name = Some(value.to_string()),
                    "count" => {
                        args.count = value.parse().unwrap_or(1);
                    }
                    _ => eprintln!("Warning: unknown key '{}'", key),
                }
            } else {
                match key_value {
                    "verbose" => args.verbose = true,
                    "help" => args.help = true,
                    _ => eprintln!("Warning: unknown flag '{}'", key_value),
                }
            }
        } else if arg.starts_with('-') && !arg.starts_with("--") {
            // Short flags: -v, -h
            for ch in arg.chars().skip(1) {
                match ch {
                    'v' => args.verbose = true,
                    'h' => args.help = true,
                    _ => eprintln!("Warning: unknown short flag '-{}'", ch),
                }
            }
        } else {
            // Positional argument
            if args.name.is_none() {
                args.name = Some(arg);
            }
        }
    }

    args
}

fn main() {
    let args = parse_args();

    if args.help {
        println!("Usage: 01_cli_args [OPTIONS] [name]");
        println!();
        println!("Options:");
        println!("  --name=NAME        Set name (or pass as positional arg)");
        println!("  --count=N          Repeat count (default: 1)");
        println!("  -v, --verbose      Enable verbose output");
        println!("  -h, --help         Show this help");
        println!();
        println!("Examples:");
        println!("  ./01_cli_args --name=Alice --verbose --count=3");
        println!("  ./01_cli_args -v Alice");
        return;
    }

    let name = args.name.as_deref().unwrap_or("World");
    for i in 0..args.count {
        let prefix = if args.verbose {
            format!("[{}/{}] ", i + 1, args.count)
        } else {
            String::new()
        };
        println!("{}Hello, {}!", prefix, name);
    }

    if args.verbose {
        eprintln!("Debug: parsed args = {:?}", args);
    }
}
