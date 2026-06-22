# 15 Testing — Making sure your code works

> Rust's testing story is built-in. No framework needed. `#[test]`, `assert!`, and `cargo test` are all you need to start.

---

## Cheatsheet

```rust
// --- Basic test ---
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert!(true);
    assert!(result.is_ok());
}

// --- Test module (only compiled during test) ---
#[cfg(test)]
mod tests {
    use super::*;  // import parent module's functions

    #[test]
    fn test_private_fn() {
        // Can test private functions from unit tests!
        assert_eq!(private_helper(5), 10);
    }
}

// --- Tests returning Result (for ? operator) ---
#[test]
fn test_parse() -> Result<(), ParseIntError> {
    let n: i32 = "42".parse()?;
    assert_eq!(n, 42);
    Ok(())
}

// --- Doc tests (in /// documentation comments) ---
/// Adds two numbers.
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }

// --- Integration tests (tests/ directory) ---
// tests/integration.rs:
// use my_crate::my_function;
// #[test]
// fn test_from_outside() { /* ... */ }

// --- Run tests ---
// cargo test                  # all tests
// cargo test test_name        # filter by name
// cargo test -- --nocapture   # show println! output
// rustc --test file.rs -o /tmp/t && /tmp/t
```

---

## Concepts (in order, compile and test each)

```bash
cd 15_testing
rustc --test --edition 2021 concept/01_test_basics.rs -o /tmp/t1 && /tmp/t1
rustc --test --edition 2021 concept/02_test_module.rs -o /tmp/t2 && /tmp/t2
rustc --test --edition 2021 concept/03_result_test.rs -o /tmp/t3 && /tmp/t3
# 04_doc_test.rs needs cargo test (doc tests)
rustc --test --edition 2021 concept/05_test_org.rs -o /tmp/t5 && /tmp/t5
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_test_basics.rs` | Test basics | `#[test]`, `assert!`, `assert_eq!`, running with rustc --test |
| 02 | `02_test_module.rs` | Test modules | `#[cfg(test)]`, `mod tests { use super::*; }` |
| 03 | `03_result_test.rs` | Result in tests | Tests returning `Result<()>`, `?` operator in tests |
| 04 | `04_doc_test.rs` | Doc tests | `/// ```rust blocks`, cargo test runs doc examples |
| 05 | `05_test_org.rs` | Test organization | Unit vs integration, tests/ directory, testing private fns |

---

## Exercises

```bash
cd 15_testing
make run      # compile and run all exercise tests
make test     # also runs project cargo test
```

| # | File | What it tests | Key edge cases |
|---|------|--------------|----------------|
| 01 | `ex01_math.rs` | factorial, gcd, lcm | 0!, gcd(0,5), lcm(0,5), commutative property |
| 02 | `ex02_string.rs` | truncate function | empty, exact, Unicode (🦀), max < 4 |
| 03 | `ex03_median.rs` | median of i32 slice | odd, even, empty, single, negative |
| 🏆 | `ex04_bonus_property.rs` | Property-based testing | reverse_twice, length preserved, palindrome properties |

---

## Project — calc

A calculator with library and binary. REPL interface. Full test suite.

```
cargo run -- 3 + 4    # single expression
cargo run             # interactive REPL
cargo test            # run all tests (unit + integration)
```

### Library (`src/lib.rs`)
- `Calculator` struct with `add`, `sub`, `mul`, `divide` (returns Result)
- Memory operations: `store`, `recall`, `clear_memory`, `add_to_memory`
- `eval_expression` for parsing "3 + 4" strings

### Binary (`src/main.rs`)
- REPL: type expressions, see results
- Commands: `memory`, `store <n>`, `recall`, `clear`, `add <n>`
- Non-interactive: `cargo run -- 3 + 4`

### Integration tests (`tests/integration.rs`)
- Test the public API as an external consumer
- Realistic memory workflow test

When you see all tests pass, think:
**"Every function has a test. Every edge case is covered. No regressions."**

---

## What's next?

Review any group, or start building your own Rust projects!
