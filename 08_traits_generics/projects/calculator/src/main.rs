// src/main.rs — Generic Calculator
//
// A calculator that works with i32, f64, and Complex numbers using traits.
//
// Usage:
//   cargo run -- add 5 3
//   cargo run -- sub 10.5 2.5
//   cargo run -- mul "3+4i" "1+2i"
//   cargo run -- help

mod complex;

use std::env;
use std::fmt::Display;
use std::process;
use std::str::FromStr;

use complex::Complex;

// ── The Calculator trait ─────────────────────────────────────────────────────

/// A trait for basic arithmetic operations that a calculator can perform.
trait Calculator: Display + Sized {
    /// Parse a string into Self.
    fn parse(s: &str) -> Result<Self, String>;

    /// Add two values.
    fn add(self, other: Self) -> Self;

    /// Subtract the other value from self.
    fn sub(self, other: Self) -> Self;

    /// Multiply two values.
    fn mul(self, other: Self) -> Self;

    /// Divide self by other. Returns Err on division by zero.
    fn div(self, other: Self) -> Result<Self, String>;
}

// ── Implementations ──────────────────────────────────────────────────────────

impl Calculator for i32 {
    fn parse(s: &str) -> Result<Self, String> {
        s.parse().map_err(|e| format!("invalid integer '{s}': {e}"))
    }

    fn add(self, other: Self) -> Self {
        self + other
    }

    fn sub(self, other: Self) -> Self {
        self - other
    }

    fn mul(self, other: Self) -> Self {
        self * other
    }

    fn div(self, other: Self) -> Result<Self, String> {
        if other == 0 {
            Err("division by zero".into())
        } else {
            Ok(self / other)
        }
    }
}

impl Calculator for f64 {
    fn parse(s: &str) -> Result<Self, String> {
        s.parse().map_err(|e| format!("invalid float '{s}': {e}"))
    }

    fn add(self, other: Self) -> Self {
        self + other
    }

    fn sub(self, other: Self) -> Self {
        self - other
    }

    fn mul(self, other: Self) -> Self {
        self * other
    }

    fn div(self, other: Self) -> Result<Self, String> {
        if other.abs() < f64::EPSILON {
            Err("division by zero".into())
        } else {
            Ok(self / other)
        }
    }
}

impl Calculator for Complex {
    fn parse(s: &str) -> Result<Self, String> {
        Complex::from_str(s).map_err(|e| format!("invalid complex '{s}': {e}"))
    }

    fn add(self, other: Self) -> Self {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }

    fn sub(self, other: Self) -> Self {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }

    fn mul(self, other: Self) -> Self {
        // (a+bi)(c+di) = (ac-bd) + (ad+bc)i
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }

    fn div(self, other: Self) -> Result<Self, String> {
        let denom = other.real * other.real + other.imag * other.imag;
        if denom.abs() < f64::EPSILON {
            return Err("division by zero (complex)".into());
        }
        // (a+bi)/(c+di) = (ac+bd)/(c²+d²) + (bc-ad)/(c²+d²)i
        Ok(Complex {
            real: (self.real * other.real + self.imag * other.imag) / denom,
            imag: (self.imag * other.real - self.real * other.imag) / denom,
        })
    }
}

// ── Generic calculation engine ───────────────────────────────────────────────

fn calculate<T: Calculator>(op: &str, a: T, b: T) -> Result<String, String> {
    match op {
        "add" | "+" => Ok(a.add(b).to_string()),
        "sub" | "-" => Ok(a.sub(b).to_string()),
        "mul" | "*" | "x" => Ok(a.mul(b).to_string()),
        "div" | "/" => Ok(a.div(b)?.to_string()),
        _ => Err(format!("unknown operation '{op}' — use add, sub, mul, div")),
    }
}

// ── CLI entry point ──────────────────────────────────────────────────────────

fn print_help() {
    println!(
        "\
Generic Calculator — works with integers, floats, and complex numbers.

Usage:
  calculator <operation> <a> <b>

Operations:
  add  (+)   Addition
  sub  (-)   Subtraction
  mul (*, x) Multiplication
  div  (/)   Division

Number formats:
  Integer:    5  |  -42
  Float:      3.14  |  -0.5
  Complex:    3+4i  |  1-2i  |  i  |  5i

Examples:
  calculator add 5 3
  calculator sub 10.5 2.5
  calculator mul \"3+4i\" \"1+2i\"
  calculator div 100 7
  calculator help

Options:
  help        Show this help message
  --help      Show this help message
"
    );
}

/// Determine the type based on the input strings.
/// We use a simple heuristic: if either value contains 'i', it's complex;
/// if either value contains '.', it's float; otherwise integer.
fn detect_type(a_str: &str, b_str: &str) -> &'static str {
    if a_str.contains('i') || b_str.contains('i') {
        "complex"
    } else if a_str.contains('.') || b_str.contains('.') {
        "float"
    } else {
        "int"
    }
}

fn run(args: &[String]) -> Result<String, String> {
    if args.len() < 3 {
        return Err("Usage: calculator <operation> <a> <b>".into());
    }

    let op = &args[0];
    let a_str = &args[1];
    let b_str = &args[2];

    match detect_type(a_str, b_str) {
        "int" => {
            let a = i32::parse(a_str)?;
            let b = i32::parse(b_str)?;
            calculate(op, a, b)
        }
        "float" => {
            let a = f64::parse(a_str)?;
            let b = f64::parse(b_str)?;
            calculate(op, a, b)
        }
        "complex" => {
            let a = Complex::parse(a_str)?;
            let b = Complex::parse(b_str)?;
            calculate(op, a, b)
        }
        _ => unreachable!(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Skip binary name
    let cmd_args: Vec<String> = args.into_iter().skip(1).collect();

    if cmd_args.is_empty() || cmd_args[0] == "help" || cmd_args[0] == "--help" {
        print_help();
        return;
    }

    match run(&cmd_args) {
        Ok(result) => println!("{result}"),
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    }
}
