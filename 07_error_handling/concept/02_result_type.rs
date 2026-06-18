// 02_result_type — Result<T, E>, Ok/Err, is_ok, is_err
//
// Result is the standard way to handle recoverable errors in Rust.
// It's an enum:
//
//   enum Result<T, E> {
//       Ok(T),
//       Err(E),
//   }
//
// A function that can fail returns Result — the caller must handle both cases.

use std::num::ParseIntError;

fn main() {
    // --- 1. A function that returns Result ---
    fn parse_i32(s: &str) -> Result<i32, ParseIntError> {
        s.parse::<i32>()
        // parse already returns Result<i32, ParseIntError>
    }

    // --- 2. Pattern matching on Result ---
    match parse_i32("42") {
        Ok(val) => println!("Parsed: {val}"),
        Err(e) => println!("Parse error: {e}"),
    }

    match parse_i32("not a number") {
        Ok(val) => println!("Parsed: {val}"),
        Err(e) => println!("Parse error: {e}"),
    }

    // --- 3. is_ok / is_err ---
    let good: Result<i32, &str> = Ok(42);
    let bad: Result<i32, &str> = Err("something broke");

    assert!(good.is_ok());
    assert!(!good.is_err());
    assert!(bad.is_err());
    assert!(!bad.is_ok());

    println!("good.is_ok() = {}", good.is_ok());
    println!("bad.is_err() = {}", bad.is_err());

    // --- 4. Convenience methods ---
    // ok() converts Result -> Option (discards error)
    // err() converts Result -> Option of the error
    let val_opt: Option<i32> = good.ok();
    let err_opt = bad.err();
    println!("good.ok() = {:?}", val_opt);
    println!("bad.err() = {:?}", err_opt);

    // --- 5. Creating Results manually ---
    fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
        if b.abs() < f64::EPSILON {
            Err("division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }

    match safe_divide(10.0, 2.0) {
        Ok(v) => println!("10 / 2 = {v}"),
        Err(e) => println!("Error: {e}"),
    }

    match safe_divide(10.0, 0.0) {
        Ok(v) => println!("10 / 0 = {v}"),
        Err(e) => println!("Error: {e}"),
    }

    // --- 6. Result from Option ---
    // ok_or / ok_or_else convert Option -> Result
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = opt.ok_or("value was None");
    println!("Some(42).ok_or(...) = {:?}", res);

    let none: Option<i32> = None;
    let res2: Result<i32, &str> = none.ok_or_else(|| "nothing here");
    println!("None.ok_or_else(...) = {:?}", res2);

    println!("--- 02_result_type done ---");
}
