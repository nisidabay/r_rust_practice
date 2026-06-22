fn main() {
    // Result<T, E> represents an operation that CAN fail: Ok(T) or Err(E)
    // It's Rust's primary error-handling mechanism (not exceptions)
    use std::num::ParseIntError;

    // Safe integer parser — returns Result instead of panicking
    fn parse_int(s: &str) -> Result<i32, ParseIntError> {
        s.trim().parse::<i32>()
    }

    // --- Creating Result values ---
    let ok_val: Result<i32, &str> = Ok(42);
    let err_val: Result<i32, &str> = Err("something went wrong");

    // --- match — handle both cases explicitly ---
    // The compiler forces you to deal with errors. No unhandled exceptions.
    match ok_val {
        Ok(v)  => println!("ok_val = {}", v),
        Err(e) => println!("ok_val error: {}", e),
    }

    // --- .is_ok() / .is_err() — quick boolean check ---
    println!("ok_val.is_ok() = {}", ok_val.is_ok());
    println!("err_val.is_err() = {}", err_val.is_err());

    // --- .unwrap() — get Ok OR panic (prototypes only!) ---
    println!("unwrap ok_val: {}", ok_val.unwrap());
    // println!("unwrap err_val: {}", err_val.unwrap()); // would panic!

    // --- .unwrap_or(default) — safe extraction ---
    println!("unwrap_or: {}", err_val.unwrap_or(-1));

    // --- .unwrap_or_else(closure) — compute fallback lazily ---
    let val = err_val.unwrap_or_else(|_| {
        // Only runs if Err — no wasted computation
        eprintln!("Falling back to default");
        0
    });
    println!("unwrap_or_else: {}", val);

    // --- .map(closure) — transform Ok value, leave Err untouched ---
    let doubled = ok_val.map(|v| v * 2);
    println!("map: {:?}", doubled);

    // --- .map_err(closure) — transform Err value, leave Ok untouched ---
    let mapped_err = err_val.map_err(|e| format!("ERROR: {}", e));
    println!("map_err: {:?}", mapped_err);

    // --- Practical example ---
    let parsed = parse_int("42");
    match parsed {
        Ok(n)  => println!("Parsed: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }

    let parsed = parse_int("not_a_number");
    match parsed {
        Ok(n)  => println!("Parsed: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }
}
