# 10 Errors — Handling failure the Rust way

> No exceptions. No null. No try/catch. Rust has `Result`, `panic!`, and the `?` operator — explicit, type-checked, and honest about failure.

---

## Cheatsheet

```rust
// --- panic! — crash the program (for unrecoverable states) ---
panic!("something went wrong");          // explicit panic
vec![1,2,3][100];                        // out-of-bounds panic
Option::None.unwrap();                   // panic on None
Result::Err("oops").unwrap();            // panic on Err

// --- Result<T, E> — recoverable errors ---
let r: Result<i32, &str> = Ok(42);
match r {
    Ok(v) => println!("Got {}", v),
    Err(e) => println!("Error: {}", e),
}
r.is_ok();   // true
r.is_err();  // false
r.ok();      // Some(42) — convert to Option
r.err();     // None

// --- ? operator — propagate errors upward ---
fn parse_num(s: &str) -> Result<i32, ParseIntError> {
    let n = s.trim().parse::<i32>()?;  // Early return on Err!
    Ok(n * 2)
}

// ? also works with Option:
fn first_char(s: &str) -> Option<char> {
    s.chars().next()?  // Returns None if s is empty
}

// --- Custom error types ---
#[derive(Debug)]
enum MyError {
    NotFound,
    ParseError(String),
    IoError(std::io::Error),
}

impl fmt::Display for MyError { /* ... */ }
impl std::error::Error for MyError { /* ... */ }
impl From<std::io::Error> for MyError { /* ... */ }
// Now ? auto-converts io::Error into MyError!

// --- Multiple error types: two strategies ---
// Strategy 1: Box<dyn Error> (simple, lose type info)
fn read_num(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let s = std::fs::read_to_string(path)?;
    Ok(s.trim().parse::<i32>()?)
}

// Strategy 2: Custom enum + From impls (preserve type info)
fn read_num_better(path: &str) -> Result<i32, ConfigError> {
    let s = std::fs::read_to_string(path)?;  // io::Error -> ConfigError
    Ok(s.trim().parse::<i32>()?)             // ParseIntError -> ConfigError
}
```

---

## Concepts (in order, compile and run each)

```bash
cd 10_errors
rustc --edition 2021 concept/01_panic.rs -o /tmp/p && /tmp/p
rustc --edition 2021 concept/02_result.rs -o /tmp/r && /tmp/r
rustc --edition 2021 concept/03_unwrap_expect.rs -o /tmp/ue && /tmp/ue
rustc --edition 2021 concept/04_propagate.rs -o /tmp/prop && /tmp/prop
rustc --edition 2021 concept/05_custom_error.rs -o /tmp/ce && /tmp/ce
rustc --edition 2021 concept/06_multiple_errors.rs -o /tmp/me && /tmp/me
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_panic.rs` | panic! | Crash button. Unwinds stack, runs destructors. Abort or unwind. |
| 02 | `02_result.rs` | Result<T, E> | Not exceptions. A type you MUST handle. Match, is_ok, ok/err. |
| 03 | `03_unwrap_expect.rs` | unwrap / expect | Quick value-or-panic. OK for prototypes, tests, infallible ops. |
| 04 | `04_propagate.rs` | ? operator | Early return on Err. Works with Result and Option. Zero overhead. |
| 05 | `05_custom_error.rs` | Custom error types | enum + Display + impl Error. Structured error info. |
| 06 | `06_multiple_errors.rs` | Multiple error types | Box\<dyn Error\> vs custom enum + From. |

---

## Exercises

```bash
cd 10_errors/exercises
make run
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_parse_csv.rs` | Parse CSV lines, return Result for bad lines | Result, match, error messages |
| 02 | `ex02_divide.rs` | Parse "a/b", custom DivError enum | Custom error enum, Display, ? |
| 03 | `ex03_config_reader.rs` | Parse KEY=VALUE, custom ConfigError enum | From impl, Display, Error trait, ? |
| 🏆 | `ex04_bonus_validate.rs` | Validate input (non-empty, 1-100 range) | Chained ?, From\<ParseIntError\>, multiple validation steps |

---

## Project — file_reader

Read a file with line numbers. Handles errors: file not found, permission denied, non-UTF8 content.

```bash
cd 10_errors/project
cargo run -- Cargo.toml
cargo run -- Cargo.toml --head 5
cargo run -- Cargo.toml --tail 3
cargo run -- nonexistent.txt
```

When you see the line-numbered output, think:
**"Every error is a type. Every ? is an early return. No crashes."**

---

## What's next?

Group 11 `11_traits`: Define shared behavior with traits — like interfaces but better.
