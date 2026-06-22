# 04 Ownership — moves, clones, slices, and who owns what

> Rust's most important concept. 6 concepts + 4 exercises + 1 project.

---

## Cheatsheet

```rust
// Three rules of ownership
// 1. Each value has one owner
// 2. One owner at a time
// 3. Owner drops value when it goes out of scope

// Move semantics — ownership transfers (String, Vec, etc.)
let s1 = String::from("hello");
let s2 = s1;            // s1 is INVALID now — use s2
// println!("{}", s1);  // COMPILE ERROR

// Copy types — implicit copy (i32, bool, char, f64)
let x = 5;
let y = x;              // x is still valid — copied, not moved
println!("{} {}", x, y);

// Clone — explicit deep copy
let s1 = String::from("hello");
let s2 = s1.clone();    // both valid
println!("{} {}", s1, s2);

// Slices — borrow contiguous parts without copying
let s = String::from("hello world");
let hello = &s[0..5];   // &str, borrows from s
let first_two = &s[..2];

// Passing ownership to a function
fn take(s: String) { }          // moves in, dropped at end
fn give() -> String { String::from("hi") }  // moves out

// Returning ownership
fn give_back(s: String) -> String { s }
```

---

## Concepts (in order, compile and run each)

```bash
rustc --edition 2021 concept/01_ownership_intro.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/02_move.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/03_copy.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/04_move_fn.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/05_clone.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/06_slices.rs -o /tmp/ex && /tmp/ex
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_ownership_intro.rs` | Three rules of ownership | scope, drop at end of scope, ownership at function boundaries |
| 02 | `02_move.rs` | Move semantics | String and Vec move on assignment, old var invalidated |
| 03 | `03_copy.rs` | Copy types | i32, bool, char, f64 are Copy. Clone trait for explicit deep copy |
| 04 | `04_move_fn.rs` | Passing/returning ownership | Move into function, return to get it back, tuple returns |
| 05 | `05_clone.rs` | Deep copy with .clone() | When to clone vs not, performance considerations |
| 06 | `06_slices.rs` | String and array slices | `&str` from String, `&[i32]` from Vec, range syntax `&s[0..5]` |

---

## Exercises

```bash
cd exercises
make run    # compile + run all exercises
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_word_counter.rs` | Count words from stdin via function | `&str` parameter, split_whitespace, return value |
| 02 | `ex02_string_reverse.rs` | Reverse a String manually | chars(), rev(), collect, return owned String |
| 03 | `ex03_first_word.rs` | Return first word as &str slice | `&str` return, `find(' ')`, slice ranges |
| 🏆 | `ex04_bonus_debug.rs` | Fix ownership errors with explanations | Compile errors, move semantics fixes, borrowing patterns |

---

## Project — `string_sanitizer`

Reads stdin, applies a chain of filters, prints the result.

```bash
cd project
echo "Hello  WORLD!!" | cargo run -- --lower --no-spaces
# → hello world
echo "  Rust is GREAT!  " | cargo run -- --trim --lower --alpha-only
# → rust is great
```

**Filters:**
- `--lower` — Convert to lowercase
- `--upper` — Convert to uppercase
- `--trim` — Remove leading/trailing whitespace
- `--no-spaces` — Collapse runs of whitespace to single spaces
- `--alpha-only` — Keep only a-zA-Z and spaces

Filters are applied in the order given on the command line.

---

## What's next?

Group 05 `05_references`: Borrowing, &T and &mut T, reference rules, lifetimes.
