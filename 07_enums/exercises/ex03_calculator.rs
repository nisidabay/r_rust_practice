// ex03_calculator.rs — Enum Op { Add, Sub, Mul, Div }
// Reads "3 + 4" from stdin, parses, computes, prints result
// Handles division by zero gracefully (panics? returns error? your call)

use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

// Parse a single arithmetic expression: "3 + 4" or "10 / 2"
// Returns Option<(Op, f64, f64)> — None if format is wrong
fn parse_expr(line: &str) -> Option<(Op, f64, f64)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 3 {
        return None;
    }
    let a: f64 = parts[0].parse().ok()?;
    let b: f64 = parts[2].parse().ok()?;
    let op = match parts[1] {
        "+" => Op::Add,
        "-" => Op::Sub,
        "*" | "x" => Op::Mul,
        "/" => Op::Div,
        _ => return None,
    };
    Some((op, a, b))
}

fn compute(op: &Op, a: f64, b: f64) -> Option<f64> {
    match op {
        Op::Add => Some(a + b),
        Op::Sub => Some(a - b),
        Op::Mul => Some(a * b),
        Op::Div => {
            if b == 0.0 {
                None
            } else {
                Some(a / b)
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    println!("Enter expressions like '3 + 4' (Ctrl+D to quit):");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match parse_expr(line) {
            Some((op, a, b)) => match compute(&op, a, b) {
                Some(result) => println!("= {}", result),
                None => println!("Error: division by zero"),
            },
            None => println!("Error: invalid expression (use: 3 + 4)"),
        }
    }
}
