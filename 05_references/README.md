# 05 References — borrowing, &T, &mut T, and lifetimes

> How to look at data without owning it. 5 concepts + 4 exercises + 1 project.

---

## Cheatsheet

```rust
// Immutable reference — shared read access
let s = String::from("hello");
let r1: &String = &s;     // borrow s
let r2: &String = &s;     // multiple readers allowed
println!("{} {}", r1, r2);

// Mutable reference — exclusive write access
let mut s = String::from("hello");
let r: &mut String = &mut s;
r.push_str(" world");     // modify through reference

// Rules:
// 1. Many immutable refs XOR one mutable ref
// 2. References must always be valid (no dangling)

// Dereferencing
let x = 5;
let rx = &x;
println!("{}", *rx);      // dereference to get value

// Function with reference parameters
fn read_only(s: &String) { s.len(); }
fn modify(s: &mut String) { s.push_str("!"); }

// Swapping with mutable references
fn swap(a: &mut i32, b: &mut i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

// Lifetime annotation (for when compiler needs help)
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

---

## Concepts (in order, compile and run each)

```bash
rustc --edition 2021 concept/01_immutable_ref.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/02_mutable_ref.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/03_ref_rules.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/04_dangling.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/05_ref_fn.rs -o /tmp/ex && /tmp/ex
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_immutable_ref.rs` | Immutable references &T | Borrowing, multiple readers, dereferencing with `*` |
| 02 | `02_mutable_ref.rs` | Mutable references &mut T | Exclusive writer, one mutable XOR many immutable |
| 03 | `03_ref_rules.rs` | Reference rules and fixes | Compile error patterns, scoping `{}` to fix |
| 04 | `04_dangling.rs` | Dangling prevention | Rust prevents dangling refs at compile time, fix by returning owned |
| 05 | `05_ref_fn.rs` | Functions with references | &T for reading, &mut T for writing, swap example |

---

## Exercises

```bash
cd exercises
make run    # compile + run all exercises
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_swap.rs` | Swap two i32 values | &mut T parameters, dereference |
| 02 | `ex02_capitalize.rs` | Capitalize first char of String | &mut String, chars(), replace_range |
| 03 | `ex03_find_longest.rs` | Return longer of two &str | lifetime annotations `<'a>`, &str refs |
| 🏆 | `ex04_bonus_scoped.rs` | Scoped blocks to avoid double borrow | {} scoping, NLL, function borrow pattern |

---

## Project — `json_extractor`

Extracts a field value from JSON-like lines without using serde.

```bash
cd project
echo '{"name":"Alice","age":30}' | cargo run -- name
# → Alice
echo '{"name":"Alice","age":30}' | cargo run -- age
# → 30
echo '{"name":"Alice","age":30}' | cargo run --pretty -- name
# → Input: {"name":"Alice","age":30}
#     name => Alice
#   ---
```

**Features:**
- No external crates — parses `"key":"value"` patterns manually
- Supports quoted (string) and unquoted (numeric) values
- `--pretty` flag for formatted input/output display
- Reads multiple lines from stdin

---

## What's next?

Group 06 `06_structs`: Define your own types with struct, methods, and derived traits.
