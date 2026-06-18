// counter — CLI counter tool
// Demonstrates: CLI arg parsing with std::env, type casting, ranges.
//
// Usage:
//   cargo run -- --count 10
//   cargo run -- --count 10 --step 2 --reverse
//   cargo run -- --count 256 --format hex
//   cargo run -- --help

use std::env;

fn print_help() {
    eprintln!("Usage: counter [options]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --count N       Number of items to count (default: 10)");
    eprintln!("  --step M        Step size (default: 1)");
    eprintln!("  --reverse       Count in reverse order");
    eprintln!("  --format FMT    Output format: dec (default) or hex");
    eprintln!("  --help          Show this help message");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Show help if --help or no args (with just the program name).
    if args.len() == 1 || args.contains(&"--help".to_string()) {
        print_help();
        return;
    }

    // Parse flags by walking through args.
    let mut count: u64 = 10;
    let mut step: u64 = 1;
    let mut reverse = false;
    let mut format_hex = false;

    let mut i = 1; // skip program name
    while i < args.len() {
        match args[i].as_str() {
            "--count" => {
                i += 1;
                count = args[i].parse().expect("--count requires an integer");
            }
            "--step" => {
                i += 1;
                step = args[i].parse().expect("--step requires an integer");
            }
            "--reverse" => reverse = true,
            "--format" => {
                i += 1;
                match args[i].as_str() {
                    "hex" => format_hex = true,
                    "dec" => format_hex = false,
                    other => panic!("Unknown format '{other}'; use 'dec' or 'hex'"),
                }
            }
            other => panic!("Unknown flag '{other}'; use --help"),
        }
        i += 1;
    }

    // Build the range depending on direction.
    let range: Box<dyn Iterator<Item = u64>> = if reverse {
        Box::new((1..=count).rev().step_by(step as usize))
    } else {
        Box::new((1..=count).step_by(step as usize))
    };

    // Print each value in the chosen format.
    for val in range {
        if format_hex {
            println!("{val:#x}");
        } else {
            println!("{val}");
        }
    }
}
