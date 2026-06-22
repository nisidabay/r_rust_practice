// ex02_to_base.rs — Trait ToBase { fn to_base(&self, base: u32) -> String }
//
// Implement for u32. Convert any u32 to any base (2-36).
// Use standard 0-9 then a-z for digits.
//
// Usage: echo "255" | ./ex02_to_base
// Expected: binary, octal, hex, base-3, base-36 of each number

use std::io::{self, BufRead};

trait ToBase {
    fn to_base(&self, base: u32) -> String;
}

impl ToBase for u32 {
    fn to_base(&self, base: u32) -> String {
        if *self == 0 {
            return "0".to_string();
        }

        let digits = b"0123456789abcdefghijklmnopqrstuvwxyz";
        let base = base.max(2).min(36);

        let mut n = *self;
        let mut result = Vec::new();

        while n > 0 {
            result.push(digits[(n % base) as usize]);
            n /= base;
        }

        result.reverse();
        String::from_utf8(result).unwrap()
    }
}

fn main() {
    let stdin = io::stdin();
    println!("Enter numbers (one per line, Ctrl+D to quit):");
    println!();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        match line.parse::<u32>() {
            Ok(n) => {
                println!("Number: {}", n);
                println!("  Binary:     {}", n.to_base(2));
                println!("  Octal:      {}", n.to_base(8));
                println!("  Decimal:    {}", n);
                println!("  Hex:        {}", n.to_base(16));
                println!("  Base-3:     {}", n.to_base(3));
                println!("  Base-36:    {}", n.to_base(36));
                println!();
            }
            Err(e) => {
                eprintln!("Error: '{}' is not a valid u32 — {}", line, e);
            }
        }
    }
}
