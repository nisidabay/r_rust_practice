# 09 Patterns — Safe, exhaustive unpacking of data

> Rust's pattern matching is its superpower. Like C's switch, but you can match on literals, ranges, multiple patterns, guards, and — most importantly — destructure types to extract their contents. The compiler checks everything is handled.

---

## Cheatsheet

```rust
// Literals and ranges
match x {
    0 => "zero",
    1..=5 => "small",
    6..=10 => "medium",
    _ => "large",
}

// Multiple patterns with |
match color {
    "red" | "blue" | "green" => "primary",
    _ => "other",
}

// Guards — extra condition on a pattern
match n {
    n if n < 0 => "negative",
    n if n % 2 == 0 => "even",
    _ => "odd",
}

// Destructuring tuples
let (x, y, _) = (10, 20, 30);

// Destructuring structs
let Person { name, age, .. } = person;

// Destructuring enums
match shape {
    Shape::Circle(r) => r * r * 3.14,
    Shape::Rect { w, h } => w * h,
}

// if let — concise single-variant match
if let Some(v) = opt { /* use v */ }
while let Some(top) = stack.pop() { /* loop */ }

// ref — borrow instead of move in match arms
match &opt {
    Some(ref val) => println!("{}", val),
    None => {}
}

// Nested Option/Result
match maybe_result {
    Some(Ok(val)) => println!("Got: {}", val),
    Some(Err(e)) => println!("Error: {}", e),
    None => println!("Nothing"),
}
```

---

## Concepts (in order, compile and run each)

```bash
rustc --edition 2021 concept/01_match_literals.rs -o /tmp/check && /tmp/check
rustc --edition 2021 concept/02_match_destructure.rs -o /tmp/check && /tmp/check
rustc --edition 2021 concept/03_if_let_while_let.rs -o /tmp/check && /tmp/check
rustc --edition 2021 concept/04_match_option_result.rs -o /tmp/check && /tmp/check
rustc --edition 2021 concept/05_ref_pattern.rs -o /tmp/check && /tmp/check
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_match_literals.rs` | Literals, ranges, \|, _, guards | Match is switch on steroids |
| 02 | `02_match_destructure.rs` | Destructure tuples, structs, enums | Pull values out of compound types |
| 03 | `03_if_let_while_let.rs` | if let / while let | Match one variant, loop while matching |
| 04 | `04_match_option_result.rs` | Nested Option&lt;Result&lt;T,E&gt;&gt; | Handle all 3 states: some/ok, some/err, none |
| 05 | `05_ref_pattern.rs` | ref, ref mut, & patterns | Borrow inside match (avoid moves) |

---

## Exercises

```bash
cd exercises
make run    # compile + run all exercises
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_parse_log.rs` | Parse "ERROR: msg", color-code by level | Match on string literals |
| 02 | `ex02_rgb_to_hex.rs` | Parse "#FF8800" or "rgb(255,136,0)" | if let, pattern matching |
| 03 | `ex03_arg_parser.rs` | Parse "--name=value" argv style | Match on string, split_once |
| 🏆 | `ex04_bonus_nested.rs` | Parse "(add 1 (mul 2 3))" recursively | Recursive descent with match |

---

## Project — Mini Grep

Search stdin for a pattern, highlight matches in red (ANSI codes). Like grep but minimal.

```bash
cd project
printf "hello world\nfoo bar\nhello again\n" | cargo run -- hello
printf "Hello\nHELLO\n" | cargo run -- --ignore-case --line-number hello
printf "hello\nworld\n" | cargo run -- --count hello
```

**Flags:**
- `--ignore-case` / `-i` — case-insensitive search
- `--count` / `-c` — only show match count
- `--line-number` / `-n` — prefix lines with line number

When you see highlighted matches, think:
**"Pattern matching isn't just for enums — it's for parsing everything."**

---

## What's next?

Group 10 `10_errors`: panic, Result, ? operator, custom errors — handling things that go wrong.
