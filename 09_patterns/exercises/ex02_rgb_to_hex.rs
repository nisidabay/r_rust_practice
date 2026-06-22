// ex02_rgb_to_hex.rs — Parse "#FF8800" or "rgb(255,136,0)"
// Match on format, destructure into RGB components, output the other format

use std::io::{self, BufRead};

// Parse an rgb(r,g,b) string like "rgb(255,136,0)" into Option<(u8,u8,u8)>
fn parse_rgb(s: &str) -> Option<(u8, u8, u8)> {
    let s = s.trim();
    if !s.starts_with("rgb(") || !s.ends_with(')') {
        return None;
    }
    let inner = &s[4..s.len() - 1]; // strip "rgb(" and ")"
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    let r = parts[0].trim().parse().ok()?;
    let g = parts[1].trim().parse().ok()?;
    let b = parts[2].trim().parse().ok()?;
    Some((r, g, b))
}

// Parse a hex string "#FF8800" into Option<(u8,u8,u8)>
fn parse_hex(s: &str) -> Option<(u8, u8, u8)> {
    let s = s.trim();
    if !s.starts_with('#') || s.len() != 7 {
        return None;
    }
    let r = u8::from_str_radix(&s[1..3], 16).ok()?;
    let g = u8::from_str_radix(&s[3..5], 16).ok()?;
    let b = u8::from_str_radix(&s[5..7], 16).ok()?;
    Some((r, g, b))
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim().to_string();

        // Match on format — parse accordingly
        if let Some((r, g, b)) = parse_hex(&line) {
            println!("Hex: #{:02X}{:02X}{:02X} -> rgb({},{},{})", r, g, b, r, g, b);
        } else if let Some((r, g, b)) = parse_rgb(&line) {
            println!("rgb({},{},{}) -> #{:02X}{:02X}{:02X}", r, g, b, r, g, b);
        } else {
            println!("Invalid format: {}", line);
        }
    }
}
