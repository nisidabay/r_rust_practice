// ex02_color_hex.rs — Parse "#FF8800" into an RGB enum
// Returns Option<Rgb> — Some for valid hex, None for invalid input

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

// Parse a hex color string like "#FF8800" into Option<Rgb>
// Returns None if the format is wrong or hex digits are invalid
fn parse_hex(hex: &str) -> Option<Rgb> {
    // Must start with '#', have exactly 6 hex digits after it
    if !hex.starts_with('#') || hex.len() != 7 {
        return None;
    }
    // Parse each two-hex-digit chunk: position 1-2 (red), 3-4 (green), 5-6 (blue)
    // u8::from_str_radix parses a string as a base-16 integer
    let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
    let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
    let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
    Some(Rgb { r, g, b })
}

fn main() {
    let inputs = ["#FF8800", "#000000", "#FFFFFF", "invalid", "#FF", "#GGGGGG"];
    for input in &inputs {
        match parse_hex(input) {
            Some(rgb) => println!("{} -> rgb({}, {}, {})", input, rgb.r, rgb.g, rgb.b),
            None => println!("{} -> invalid hex color", input),
        }
    }
}
