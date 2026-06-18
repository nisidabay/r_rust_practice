// ex02_color_hex.rs — Parse hex color into RGB with pattern matching
// WHY: Colors like "#FFAABB" are just strings — use pattern matching
//      on chars and slices to extract R, G, B components.

/// Parse a hex color string like "#FFAABB" into (r, g, b) as u8 values.
///
/// The string has the form "#" followed by 6 hex digits (0-9, A-F, a-f).
/// Return None if the format is invalid.
fn parse_hex_color(s: &str) -> Option<(u8, u8, u8)> {
    // TODO: Use pattern matching to validate and parse.
    // Steps:
    //   1. Check the string starts with '#' using a match on chars/bytes
    //   2. Verify remaining chars are exactly 6 hex digits
    //   3. Parse each pair of hex digits into a u8
    //      - s[1..3] is the red component as &str
    //      - u8::from_str_radix(pair, 16) parses hex
    //   4. Return Some((r, g, b)) or None on any failure
    //
    // Pattern matching ideas:
    //   - match on chars() or as_bytes() slices with .. rest pattern
    //   - Use if let with pattern matching for the hex parse
    //   - Use ? operator to short-circuit on None
    todo!("implement parse_hex_color")
}

fn main() {
    let tests = ["#FFAABB", "#000000", "#ffffff", "#123", "invalid", "#GGGGGG"];

    println!("=== Hex Color Parser ===");
    for hex in &tests {
        match parse_hex_color(hex) {
            Some((r, g, b)) => println!("{hex} => RGB({r}, {g}, {b})"),
            None => println!("{hex} => invalid"),
        }
    }
}
