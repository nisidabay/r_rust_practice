fn main() {
    // The ? operator — propagate errors UP to the caller.
    // If Result is Ok(v), ? unwraps to v.
    // If Result is Err(e), ? returns Err(e) from the current function EARLY.
    // This is Rust's answer to exception propagation — explicit, type-checked,
    // zero overhead.

    use std::num::ParseIntError;
    use std::fs;

    // --- Basic ? usage ---
    fn parse_num(s: &str) -> Result<i32, ParseIntError> {
        // Without ?:  let n = s.trim().parse::<i32>()?;
        // Without ?:  let n = match s.trim().parse::<i32>() { Ok(n) => n, Err(e) => return Err(e) };
        let n = s.trim().parse::<i32>()?;  // Early return on error!
        Ok(n * 2)
    }

    println!("parse_num: {:?}", parse_num("21"));   // Ok(42)
    println!("parse_num: {:?}", parse_num("abc"));  // Err(...)

    // --- Chaining ? ---
    fn read_and_parse(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
        // Chain three fallible operations with ?
        let contents = fs::read_to_string(path)?;      // io::Error
        let num = contents.trim().parse::<i32>()?;     // ParseIntError
        Ok(num * 10)
    }

    // Try with a real file:
    match read_and_parse("/etc/hostname") {
        Ok(val) => println!("Hostname number: {}", val),
        Err(e) => println!("read_and_parse error: {}", e),
    }

    // --- ? with Option ---
    fn get_first_char(s: &str) -> Option<char> {
        // ? also works with Option — returns None on None
        let c = s.chars().next()?;  // Returns None if s is empty
        Some(c.to_ascii_uppercase())
    }

    println!("first_char: {:?}", get_first_char("hello")); // Some('H')
    println!("first_char: {:?}", get_first_char(""));      // None

    // --- The key insight ---
    println!("\n? is NOT try/catch. It's an explicit, type-checked early return.");
    println!("  - If fn returns Result<_, E>, ? requires Result<_, E> expressions");
    println!("  - If fn returns Option<T>,  ? requires Option<T> expressions");
    println!("  - Different error types need Box<dyn Error> or custom conversion");
}
