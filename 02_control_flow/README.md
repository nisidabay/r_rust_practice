# 02 Control Flow — if, loop, while, for, match, if let

**One Question:** *How does Rust decide what runs next?*

## Quick Start

```bash
# Run concept examples
rustc --edition 2021 concept/01_if_else.rs && ./01_if_else
rustc --edition 2021 concept/02_loop.rs && ./02_loop
rustc --edition 2021 concept/03_while.rs && ./03_while
rustc --edition 2021 concept/04_for.rs && ./04_for
rustc --edition 2021 concept/05_match_basics.rs && ./05_match_basics
rustc --edition 2021 concept/06_loop_labels.rs && ./06_loop_labels
rustc --edition 2021 concept/07_if_let.rs && ./07_if_let

# Run exercises
cd exercises && make run

# Run project (interactive CLI)
cd project && cargo run -- --difficulty easy
cargo run -- --difficulty medium
cargo run -- --difficulty hard
cargo run -- --help
```

## Learning Path

| Step | File | Concept | Key Takeaway |
|------|------|---------|-------------|
| 1 | `01_if_else.rs` | `if`/`else if`/`else` | `if` is an **expression**, not a statement — it can return a value. Conditions must be `bool`. |
| 2 | `02_loop.rs` | `loop`, `break`, `continue` | Infinite `loop` with `break` (optionally returning a value). `continue` skips to the next iteration. |
| 3 | `03_while.rs` | `while`, `while let` | Repeats while a boolean condition holds. `while let` keeps running while pattern matching succeeds. |
| 4 | `04_for.rs` | `for`, ranges, iterators | Preferred loop: `for x in 0..n` (exclusive) or `0..=n` (inclusive). Works with any `IntoIterator`. |
| 5 | `05_match_basics.rs` | `match` | Powerful pattern matching: must be **exhaustive**. Supports `\|` (or), `..=` (ranges), `_` (catch-all). |
| 6 | `06_loop_labels.rs` | Loop labels | `'label:` names a loop. `break 'label` / `continue 'label` targets a specific outer loop. |
| 7 | `07_if_let.rs` | `if let` | Concise single-pattern matching. Use when you only care about **one variant** of an enum. |

## Exercises

| Exercise | File | Description | Skills |
|----------|------|-------------|--------|
| 1 | `ex01_fizzbuzz.rs` | Classic FizzBuzz | `for`, `match` on tuples, ranges |
| 2 | `ex02_guessing.rs` | Number guessing game | `loop`, `match` on `Ordering`, `stdin`, `parse` |
| 3 | `ex03_roman.rs` | Roman numerals 1-100 | `for`, decomposition, lookup tables |
| BONUS | `ex04_collatz.rs` | Collatz sequence length | `while`, `match`, `for`, iteration |

## Common Patterns

```rust
// Pattern 1: if as expression (ternary replacement)
let result = if cond { a } else { b };

// Pattern 2: loop returning a value
let value = loop { break 42; };

// Pattern 3: while let with Option
while let Some(x) = iter.next() { /* use x */ }

// Pattern 4: for with enumerate
for (i, item) in collection.iter().enumerate() { /* use i and item */ }

// Pattern 5: match with guards
match value {
    x if x > 0 => "positive",
    x if x < 0 => "negative",
    _ => "zero",
}

// Pattern 6: if let for single-variant concern
if let Some(val) = optional { /* use val */ }

// Pattern 7: labeled break from nested loops
'outer: for x in 0..n {
    for y in 0..m {
        if condition { break 'outer; }
    }
}

// Pattern 8: match on tuple for multi-condition logic
match (a % 3, a % 5) {
    (0, 0) => "FizzBuzz",
    (0, _) => "Fizz",
    (_, 0) => "Buzz",
    _ => "",
}
```

## Project: Math Quiz

An interactive CLI math quiz tool in `project/`. It uses `loop`, `match`, `for`, and `if let` throughout.

```bash
cd project
cargo run -- --difficulty easy    # Numbers 1-9
cargo run -- --difficulty medium  # Numbers 1-20
cargo run -- --difficulty hard    # Numbers 1-50
```

Features: random question generation with a simple PRNG, score tracking, difficulty levels, quit command, and EOF handling.

## Now Build Your Own

1. **Prime checker**: Write a program that checks if a number is prime using `for` and `while` loops. Use `match` to print "prime", "composite", or "1 is special".
2. **Multiplication table**: Print a 10×10 multiplication table using nested `for` loops with `{:4}` formatting.
3. **Word frequency**: Read lines from stdin, count word occurrences using a `loop` with `match` on `Result`, and print results.
4. **Fibonacci sequence**: Generate the first 20 Fibonacci numbers using a `loop` with `break` returning a `Vec`.
