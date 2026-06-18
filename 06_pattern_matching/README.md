# 06 — Pattern Matching

## One Question
**How do you safely unpack any shape of data?**

## Quick Start
```bash
# Compile and run any concept file
rustc --edition 2021 concept/01_match_basics.rs && ./01_match_basics

# Build and test the json_query project
cd project && cargo build && cargo test

# Run all checks
make check
```

## Learning Path
1. **01_match_basics** — Match on integers, catch-all with `_`, exhaustiveness guarantee
2. **02_match_destructure** — Destructure tuples, structs, and enum variants in match arms
3. **03_match_guards** — Add boolean conditions (`if`) to pattern arms
4. **04_match_ranges** — `..=` range patterns and `@` bindings for naming sub-patterns
5. **05_if_let_while_let** — Concise single-pattern matching with `if let`, `while let`, `let else`
6. **06_match_option_result** — Idiomatic matching on the two most common enums
7. **07_pattern_syntax** — All pattern features in one file: `|`, `..`, `_`, `ref`, `mut`, `@`

## Common Patterns
```rust
// Catch-all arm with _
match val { 0 => "zero", _ => "non-zero" }

// Destructuring a tuple
match pair { (0, y) => format!("x=0, y={y}"), (x, 0) => format!("x={x}, y=0"), _ => "other" }

// Guard to filter
match n { x if x > 0 => "positive", _ => "non-positive" }

// Range with @ binding
match score { n @ 90..=100 => format!("Excellent: {n}"), _ => "keep trying" }

// if let for single-variant interest
if let Some(val) = optional { println!("got {val}"); }

// let else for early return
let Ok(data) = result else { return Err("failed"); };

// | combiner for OR patterns
match c { 'a' | 'e' | 'i' | 'o' | 'u' => "vowel", _ => "consonant" }

// .. rest pattern in slices/structs
match arr { [first, .., last] => println!("{first}, {last}") }
```

## Now Build Your Own
Write a **command-line calculator** that evaluates expressions like `"3 + 4 * 2"` with operator precedence. Use pattern matching to:
- Parse tokens: numbers, operators, parentheses
- Match operator precedence levels
- Evaluate sub-expressions recursively

**Stretch goal**: Add variables: `"let x = 5"` and `"x + 3"` using pattern matching on the parsed AST.
