fn main() {
    // Deep matching on nested Option and Result
    // Real code often has Option<Result<T,E>> or Result<Option<T>,E>

    // --- Option<Result<T,E>> ---
    // A value that might be absent, and if present, might have failed
    let data: Option<Result<i32, String>> = Some(Ok(42));
    let missing: Option<Result<i32, String>> = None;
    let failed: Option<Result<i32, String>> = Some(Err("network error".to_string()));

    // Nested match — handle ALL combinations
    for item in &[data, missing, failed] {
        match item {
            Some(Ok(val)) => println!("Got value: {}", val),
            Some(Err(e)) => println!("Operation failed: {}", e),
            None => println!("No data available"),
        }
    }

    // --- Result<Option<T>, E> ---
    // An operation that might fail, and if it succeeds, might have no result
    let found: Result<Option<i32>, String> = Ok(Some(42));
    let not_found: Result<Option<i32>, String> = Ok(None);
    let error: Result<Option<i32>, String> = Err("database error".to_string());

    for item in &[found, not_found, error] {
        match item {
            Ok(Some(val)) => println!("Found: {}", val),
            Ok(None) => println!("Query succeeded but no result"),
            Err(msg) => println!("Error: {}", msg),
        }
    }

    // --- Nested Option (Option<Option<T>>) ---
    let double_some: Option<Option<i32>> = Some(Some(42));
    let single_some: Option<Option<i32>> = Some(None);
    let double_none: Option<Option<i32>> = None;

    for item in &[double_some, single_some, double_none] {
        match item {
            Some(Some(v)) => println!("Deep value: {}", v),
            Some(None) => println!("Shallow None"),
            None => println!("Deep None"),
        }
    }
}
