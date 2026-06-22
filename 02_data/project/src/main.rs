/*
 * unit_converter — CLI tool to convert between units
 *
 * Usage: echo "150 cm to m" | cargo run --
 *        echo "5 kg to g"   | cargo run --
 *        echo "2 h to min"  | cargo run --
 *
 * Supports:
 *   Length: mm, cm, m, km, in, ft
 *   Weight: mg, g, kg, lb
 *   Time:   s, min, h, day
 */

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }

        // Parse: "150 cm to m"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 || parts[2] != "to" {
            eprintln!("Usage: <value> <from_unit> to <to_unit>");
            eprintln!("Example: 150 cm to m");
            continue;
        }

        let value: f64 = match parts[0].parse() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Invalid number '{}': {}", parts[0], e);
                continue;
            }
        };
        let from = parts[1];
        let to = parts[3];

        let result = convert(value, from, to);
        match result {
            Some(r) => println!("{} {} = {} {}", value, from, r, to),
            None => eprintln!("Conversion not supported: {} → {}", from, to),
        }
    }
}

/// Convert a value from one unit to another.
/// Returns None if the units are incompatible or unknown.
fn convert(value: f64, from: &str, to: &str) -> Option<f64> {
    // Define unit categories with their conversion factors to base unit
    #[derive(Clone, Copy)]
    struct UnitInfo {
        category: &'static str,
        to_base: f64, // multiply by this to convert to base unit
    }

    // Base units: m, g, s
    let units = [
        // Length
        ("mm", UnitInfo { category: "length", to_base: 0.001 }),
        ("cm", UnitInfo { category: "length", to_base: 0.01 }),
        ("m",  UnitInfo { category: "length", to_base: 1.0 }),
        ("km", UnitInfo { category: "length", to_base: 1000.0 }),
        ("in", UnitInfo { category: "length", to_base: 0.0254 }),
        ("ft", UnitInfo { category: "length", to_base: 0.3048 }),
        // Weight
        ("mg", UnitInfo { category: "weight", to_base: 0.001 }),
        ("g",  UnitInfo { category: "weight", to_base: 1.0 }),
        ("kg", UnitInfo { category: "weight", to_base: 1000.0 }),
        ("lb", UnitInfo { category: "weight", to_base: 453.592 }),
        // Time
        ("s",   UnitInfo { category: "time", to_base: 1.0 }),
        ("min", UnitInfo { category: "time", to_base: 60.0 }),
        ("h",   UnitInfo { category: "time", to_base: 3600.0 }),
        ("day", UnitInfo { category: "time", to_base: 86400.0 }),
    ];

    // Look up from-unit and to-unit
    let from_info = units.iter().find(|(name, _)| *name == from)?;
    let to_info = units.iter().find(|(name, _)| *name == to)?;

    // Must be same category
    if from_info.1.category != to_info.1.category {
        return None;
    }

    // Convert: value → base → target
    let base_value = value * from_info.1.to_base;
    let result = base_value / to_info.1.to_base;

    // Round to avoid floating point noise
    Some((result * 1e10).round() / 1e10)
}
