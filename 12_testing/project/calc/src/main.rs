//! # calc — A CLI calculator built with TDD
//!
//! Every arithmetic operation has tests first.
//!
//! ## Usage
//!
//! ```bash
//! cargo run -- add 2 3
//! cargo run -- sub 10 4
//! cargo run -- mul 3 7
//! cargo run -- div 10 2
//! cargo run -- --help
//! ```

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        print_help();
        return;
    }

    if args.len() != 4 {
        eprintln!("Error: expected 3 arguments (op a b), got {}", args.len() - 1);
        eprintln!("Usage: calc <op> <a> <b>");
        process::exit(1);
    }

    let op = &args[1];
    let a: i32 = match args[2].parse() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error: cannot parse '{}' as integer: {}", args[2], e);
            process::exit(1);
        }
    };
    let b: i32 = match args[3].parse() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error: cannot parse '{}' as integer: {}", args[3], e);
            process::exit(1);
        }
    };

    let result = match op.as_str() {
        "add" => calc::add(a, b).to_string(),
        "sub" => calc::sub(a, b).to_string(),
        "mul" => calc::mul(a, b).to_string(),
        "div" => match calc::div(a, b) {
            Some(n) => n.to_string(),
            None => {
                eprintln!("Error: division by zero (or overflow)");
                process::exit(1);
            }
        },
        _ => {
            eprintln!("Error: unknown operation '{op}'. Use: add, sub, mul, div");
            process::exit(1);
        }
    };

    println!("{result}");
}

fn print_help() {
    println!("calc — A CLI calculator");
    println!();
    println!("Usage:");
    println!("  calc add  <a> <b>   Add two integers");
    println!("  calc sub  <a> <b>   Subtract b from a");
    println!("  calc mul  <a> <b>   Multiply two integers");
    println!("  calc div  <a> <b>   Divide a by b");
    println!("  calc --help         Show this help message");
    println!();
    println!("Examples:");
    println!("  calc add 2 3        => 5");
    println!("  calc div 10 2       => 5");
}
