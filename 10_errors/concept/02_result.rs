fn main() {
    // Result<T, E> — Rust's primary error handling mechanism.
    // Not exceptions. Not error codes. A type you MUST handle.
    use std::fs::File;
    use std::io::ErrorKind;

    // --- Creating and matching Result ---
    let ok: Result<i32, &str> = Ok(42);
    let err: Result<i32, &str> = Err("failed");

    // Match — exhaustive (compiler forces you to handle both cases)
    match ok {
        Ok(v) => println!("Got {}", v),
        Err(e) => println!("Error: {}", e),
    }

    // --- Practical: opening a file ---
    // File::open returns Result<File, std::io::Error>
    let file_result = File::open("nonexistent.txt");

    match file_result {
        Ok(_file) => println!("File opened"),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("File not found — handle gracefully"),
            ErrorKind::PermissionDenied => println!("Permission denied"),
            other => println!("Other error: {:?}", other),
        },
    }

    // --- .is_ok() / .is_err() — quick boolean check ---
    let r: Result<i32, &str> = Ok(10);
    println!("is_ok: {}, is_err: {}", r.is_ok(), r.is_err());

    // --- .ok() / .err() — convert to Option ---
    // .ok() turns Ok(v) into Some(v), Err(e) into None
    // .err() turns Err(e) into Some(e), Ok(v) into None
    println!("ok() -> {:?}", r.ok());   // Some(10)
    println!("err() -> {:?}", r.err()); // None
}
