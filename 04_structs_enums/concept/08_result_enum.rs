// Result<T, E> represents an operation that can succeed (Ok) or fail (Err).
// It's defined as: enum Result<T, E> { Ok(T), Err(E) }
// This is Rust's primary error-handling mechanism.

fn main() {
    // --- Creating Result values ---
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");

    println!("success: {:?}", success);
    println!("failure: {:?}", failure);

    // --- Matching on Result ---
    fn parse_number(s: &str) -> Result<i32, String> {
        match s.parse::<i32>() {
            Ok(n) => Ok(n),
            Err(e) => Err(format!("Parse error: {e}")),
        }
    }

    match parse_number("42") {
        Ok(n) => println!("Parsed: {n}"),
        Err(e) => println!("Error: {e}"),
    }

    match parse_number("hello") {
        Ok(n) => println!("Parsed: {n}"),
        Err(e) => println!("Error: {e}"),
    }

    // --- unwrap and expect (use carefully!) ---
    let ok_val = Ok::<i32, &str>(10);
    println!("Unwrapped: {}", ok_val.unwrap());

    // This would panic:
    // let err_val: Result<i32, &str> = Err("oops");
    // err_val.unwrap();  // panics!

    let ok_val2 = Ok::<i32, &str>(20);
    println!("Expected: {}", ok_val2.expect("should be ok"));

    // --- is_ok / is_err ---
    let r: Result<i32, &str> = Ok(5);
    if r.is_ok() {
        println!("Result is Ok");
    }

    // --- map and unwrap_or_else ---
    let r1: Result<i32, &str> = Ok(10);
    let r2: Result<i32, &str> = Err("fail");

    let mapped1 = r1.map(|n| n * 2);
    let mapped2 = r2.map(|n| n * 2);
    println!("Mapped Ok: {:?}, Mapped Err: {:?}", mapped1, mapped2);

    // unwrap_or_else — provide fallback from error
    let val1 = r1.unwrap_or_else(|_| 0);
    let val2 = r2.unwrap_or_else(|_| 0);
    println!("val1: {val1}, val2: {val2}");

    // --- Real-world: file-like read ---
    #[derive(Debug)]
    enum ReadError {
        NotFound,
        PermissionDenied,
    }

    fn read_config(path: &str) -> Result<String, ReadError> {
        if path == "config.toml" {
            Ok(String::from("key = \"value\""))
        } else if path == "secret.toml" {
            Err(ReadError::PermissionDenied)
        } else {
            Err(ReadError::NotFound)
        }
    }

    match read_config("config.toml") {
        Ok(content) => println!("Config: {content}"),
        Err(e) => println!("Error: {:?}", e),
    }

    match read_config("missing.toml") {
        Ok(content) => println!("Config: {content}"),
        Err(e) => println!("Error: {:?}", e),
    }
}
