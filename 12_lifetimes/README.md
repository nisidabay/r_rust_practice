# 12 Lifetimes — How Rust ensures references are always valid

> Lifetimes are Rust's way of proving that every reference points to memory that's still alive. No garbage collector, no dangling pointers. Just compile-time proofs.

---

## Cheatsheet

```rust
// 1. Elision — compiler fills in lifetimes for common patterns
fn first_word(s: &str) -> &str { s.split_whitespace().next().unwrap_or("") }

// 2. Explicit — when elision can't infer
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { if x.len() > y.len() { x } else { y } }

// 3. Struct with lifetime
struct Book<'a> { title: &'a str }

// 4. Static — lives for entire program
let s: &'static str = "hello";

// 5. Multiple lifetimes
fn choose<'a, 'b>(x: &'a str, y: &'b str) -> &'a str { x }
```

Three elision rules:
1. Each input reference gets its own lifetime
2. If exactly one input lifetime, apply it to all outputs
3. If `&self` or `&mut self`, apply its lifetime to all outputs

---

## Concepts (in order, compile and run each)

```bash
cd 12_lifetimes
rustc --edition 2021 01_elision.rs -o /tmp/elision && /tmp/elision
rustc --edition 2021 02_explicit.rs -o /tmp/explicit && /tmp/explicit
rustc --edition 2021 03_struct.rs -o /tmp/struct && /tmp/struct
rustc --edition 2021 04_static.rs -o /tmp/static && /tmp/static
rustc --edition 2021 05_multiple.rs -o /tmp/multiple && /tmp/multiple
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_elision.rs` | Lifetime elision rules | Compiler infers lifetimes for common patterns — 3 rules |
| 02 | `02_explicit.rs` | Explicit lifetime annotations | `<'a>` syntax, multiple params with shared lifetime |
| 03 | `03_struct.rs` | Structs with references | `struct Book<'a>`, impl blocks need lifetimes too |
| 04 | `04_static.rs` | `'static` lifetime | String literals, when to use `'static` vs when not to |
| 05 | `05_multiple.rs` | Multiple lifetime params | `<'a, 'b>`, different lifetimes, constraints |

---

## Exercises

```bash
cd 12_lifetimes/exercises
make run
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_longest.rs` | Return the longer of two strings | Explicit lifetime, &str comparison |
| 02 | `ex02_parser.rs` | Parse fields from input string | Struct with lifetime, method returning reference |
| 03 | `ex03_joiner.rs` | Join string slices with separator | Multiple lifetime params, building String |
| 🏆 | `ex04_bonus_messenger.rs` | Struct with two different lifetimes | `<'a, 'b>`, demonstrating distinct lifetimes |

---

## Project — HTML Builder

Build HTML strings using a struct that borrows tag names. Proves that struct lifetimes let you borrow configuration from elsewhere.

```bash
cd 12_lifetimes/project
cargo run --
```

When you see HTML output generated from borrowed tag names, think:
**"The struct doesn't own the tags — it borrows them, and the compiler proved they'll live long enough."**

---
