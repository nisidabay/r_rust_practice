/*
 * ex01_unit_convert.rs — Exercise 1
 *
 * Task: Read a value, from_unit, and to_unit from stdin. Convert between units.
 *
 * Run: echo "100 cm to m" | ./ex01_unit_convert
 * Expected output: 100 cm = 1.0 m
 *
 * Supported conversions: mm, cm, m, km (length) — all convert via meters
 */

use std::io::{self, Write};

fn main() {
    print!("Enter conversion (e.g., '100 cm to m'): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    // Parse: split on whitespace, expect format "VALUE FROM to TO"
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 4 || parts[2] != "to" {
        eprintln!("Usage: <value> <from_unit> to <to_unit>");
        eprintln!("Example: 100 cm to m");
        return;
    }

    let value: f64 = parts[0].parse().expect("Invalid number");
    let from = parts[1];
    let to = parts[3];

    // Convert everything to a base unit (meters) first
    fn to_meters(val: f64, unit: &str) -> f64 {
        match unit {
            "mm" => val / 1000.0,
            "cm" => val / 100.0,
            "m"  => val,
            "km" => val * 1000.0,
            "in" => val * 0.0254,
            "ft" => val * 0.3048,
            _    => { eprintln!("Unknown unit: {}", unit); std::f64::NAN }
        }
    }

    fn from_meters(val: f64, unit: &str) -> f64 {
        match unit {
            "mm" => val * 1000.0,
            "cm" => val * 100.0,
            "m"  => val,
            "km" => val / 1000.0,
            "in" => val / 0.0254,
            "ft" => val / 0.3048,
            _    => { eprintln!("Unknown unit: {}", unit); std::f64::NAN }
        }
    }

    let meters = to_meters(value, from);
    if meters.is_nan() { return; }

    let result = from_meters(meters, to);
    if result.is_nan() { return; }

    println!("{} {} = {} {}", value, from, result, to);
}
