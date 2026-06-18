# 12. Testing — `#[test]`, Assertions, Doc Tests, Integration Tests

## One Question

**How do you know your code works — and stays working?**

A program that compiles is not a program that works. Testing is how you verify behavior, catch regressions, and document intended usage all at once. Rust's built-in test framework makes it trivially easy to write and run tests alongside your code.

---

## Quick Start

```rust
// tests live right next to your code
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
```

```
$ rustc --test my_program.rs
$ ./my_program   # runs all #[test] functions
```

With Cargo:

```
$ cargo test
```

---

## Learning Path

1. **`#[test]` basics** — Annotate a function with `#[test]`, write assertions with `assert!`, `assert_eq!`, `assert_ne!`.
2. **`#[should_panic]`** — Test that code panics when expected.
3. **Test organisation** — Unit tests in the same file (`#[cfg(test)]`), integration tests in `tests/`.
4. **Returning `Result<()>` from tests** — Use `?` in tests for ergonomic error handling.
5. **Doc tests** — Code examples in `///` comments that `cargo test` runs automatically.
6. **Benchmarking** — Simple manual timing; the unstable `#[bench]` or nightly `std::hint::black_box`.
7. **Mocking** — Basic mock patterns with enums and traits when you need to fake external dependencies.

---

## Common Patterns

| Pattern | Idiom |
|---|---|
| Basic assertion | `assert_eq!(actual, expected)` |
| Custom failure msg | `assert!(cond, "msg: {}", val)` |
| Expected panic | `#[should_panic(expected = "...")]` |
| Test with `?` | `fn test() -> Result<()>` |
| Conditional compilation | `#[cfg(test)] mod tests { ... }` |
| Integration test | `tests/my_test.rs` (Cargo picks it up) |
| Doc test | `/// ``` rust\n/// assert!(true);\n/// ```` |
| Ignore slow test | `#[ignore]` |
| Test helper modules | `mod helpers;` guarded by `#[cfg(test)]` |

---

## Now Build Your Own

Work through the exercises in order:

1. **ex01_math_test** — Test `add`, `sub`, `mul`, `div` functions covering edge cases (overflow, divide-by-zero).
2. **ex02_string_test** — Test string utilities: `reverse`, `is_palindrome`, `count_words`.
3. **ex03_sort_test** — Test sort functions: empty, single, sorted, reverse-sorted, duplicates.
4. **BONUS: ex04_integration_test** — Create a library crate with both unit tests (inline) and integration tests (in `tests/`).

Then build the **`calc`** project — a CLI calculator built with TDD, where every operation has tests first.
