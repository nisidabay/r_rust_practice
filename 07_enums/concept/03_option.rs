fn main() {
    // Option<T> represents a value that MAY be present (Some) or absent (None)
    // It's Rust's way of avoiding NULL pointers — you MUST check before using
    
    // Safe division — returns Some(quotient) or None for divide-by-zero
    fn safe_div(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 { None } else { Some(a / b) }
    }

    // --- Creating Option values ---
    let some_val: Option<i32> = Some(42);
    let none_val: Option<i32> = None;

    // --- match — the thorough way ---
    match some_val {
        Some(v) => println!("some_val = {}", v),
        None    => println!("some_val is None"),
    }

    // --- .is_some() / .is_none() — quick boolean check ---
    println!("some_val.is_some() = {}", some_val.is_some());
    println!("none_val.is_none() = {}", none_val.is_none());

    // --- .unwrap() — get the value OR panic (use only when you're SURE) ---
    println!("unwrap some_val: {}", some_val.unwrap());
    // println!("unwrap none_val: {}", none_val.unwrap()); // would panic!

    // --- .unwrap_or(default) — safe extraction with fallback ---
    println!("unwrap_or: {}", none_val.unwrap_or(0));

    // --- .map(closure) — transform Some value, leave None untouched ---
    let doubled = some_val.map(|v| v * 2);
    println!("map double: {:?}", doubled);

    // --- Practical example ---
    let result = safe_div(10.0, 2.0); // Some(5.0)
    match result {
        Some(q) => println!("10 / 2 = {}", q),
        None    => println!("Division by zero!"),
    }

    let result = safe_div(10.0, 0.0); // None
    match result {
        Some(q) => println!("10 / 0 = {}", q),
        None    => println!("Division by zero!"),
    }
}
