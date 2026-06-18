# 07 — Error Handling

Rust's approach to errors is **explicit, type-safe, and composable**. Instead of exceptions (which can be thrown from anywhere), Rust uses
`Result<T, E>` for recoverable errors and `panic!` for unrecoverable ones. Every fallible function declares its error type in its
signature, so callers know exactly what to handle.

## Quick Start

```bash
# Run the concept demos
cd concept/
rustc --edition 2021 01_panic.rs -o 01_panic && ./01_panic
rustc --edition 2021 02_result_type.rs -o 02_result_type && ./02_result_type
# ... etc

# Run the exercises
cd ../exercises/
make check           # compiles & runs exercises 1-3
make run_ex04        # runs the bonus exercise
```

## Learning Path

| #  | File | Concept | Key Takeaways |
|----|------|---------|---------------|
| 1  | `01_panic.rs` | `panic!` and unwinding | When (not) to crash, `catch_unwind` |
| 2  | `02_result_type.rs` | `Result<T, E>` | Ok/Err, is_ok/is_err, convenience methods |
| 3  | `03_unwrap_expect.rs` | unwrap / expect | Why they're dangerous, safe alternatives |
| 4  | `04_propagate.rs` | `?` operator | Early propagation, auto-conversion |
| 5  | `05_custom_errors.rs` | Custom error types | Defining enums, Display, Error trait |
| 6  | `06_multiple_errors.rs` | Multiple error types | Box\<dyn Error\>, custom enum + From impls |
| 7  | `07_option_result_combine.rs` | Combinators | map, and_then, or_else, unwrap_or |
| 8  | `08_try_blocks.rs` | Try blocks & patterns | Closure-as-try, collect, helper fns |

## Common Patterns

### Fail-fast with `?`

```rust
fn read_score(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;  // io::Error -> Box<dyn Error>
    let score: i32 = content.trim().parse()?;       // ParseIntError -> Box<dyn Error>
    Ok(score)
}
```

### Collecting all errors (validation)

```rust
fn validate_all(items: &[&str]) -> Result<Vec<i32>, Vec<String>> {
    let mut vals = vec![];
    let mut errs = vec![];
    for item in items {
        match item.parse::<i32>() {
            Ok(v) => vals.push(v),
            Err(e) => errs.push(format!("{item}: {e}")),
        }
    }
    if errs.is_empty() { Ok(vals) } else { Err(errs) }
}
```

### Custom error wrapping

```rust
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    InvalidInput(String),
}

impl From<std::io::Error> for AppError { /* ... */ }
impl From<std::num::ParseIntError> for AppError { /* ... */ }
// Now `?` converts io::Error and ParseIntError automatically
```

## Now Build Your Own

Complete the exercises in `exercises/`:

1. **ex01** — Parse a string to i32, return a descriptive error with `map_err`
2. **ex02** — Safe division + cumulative product with `?` propagation
3. **ex03** — Read a key=value config file, validate each line with custom error messages
4. **ex04 (bonus)** — Validate CSV data, collecting *all* errors before returning

Then build the **todo CLI** in `project/todo/` — a persistent task manager using `Result` throughout.

Ready? Head to `exercises/` and run `make check`!
