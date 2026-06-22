# 03 Control — if/else, loops, match, and control flow

> Control the flow of your program. 5 concepts + 4 exercises + 1 project.

---

## Cheatsheet

```rust
// if/else is an EXPRESSION — it returns a value
let status = if age >= 18 { "adult" } else { "minor" };

// loop — infinite loop (must break to exit)
loop {
    if done { break; }
}

// loop with break returning a value
let result = loop { break 42; };

// while — runs while condition is true
while n > 0 { n -= 1; }

// while let — destructure until None
while let Some(val) = iter.next() { /* use val */ }

// for — iterate over ranges and collections
for i in 0..10 { }           // exclusive range: 0-9
for i in 0..=10 { }          // inclusive range: 0-10
for item in &collection { }  // by reference
for (i, item) in iter.enumerate() { }  // with index

// match — exhaustive pattern matching (also an expression)
match value {
    1 => "one",
    2..=5 => "small range",
    _ => "catch-all",
}
```

---

## Concepts (in order, compile and run each)

```bash
rustc --edition 2021 concept/01_if_else.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/02_loop.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/03_while.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/04_for.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/05_match.rs -o /tmp/ex && /tmp/ex
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_if_else.rs` | if/else as expression | `if` returns a value, `else if` chains, no ternary needed |
| 02 | `02_loop.rs` | Infinite loops | `loop`, `break` with value, `continue`, loop labels `'outer:` |
| 03 | `03_while.rs` | Conditional loops | `while`, `while let` destructuring `Option`, `pop()` |
| 04 | `04_for.rs` | For loops | ranges `0..n`, `0..=n`, `enumerate()`, `step_by()` |
| 05 | `05_match.rs` | Pattern matching | exhaustive arms, ranges `..=`, `\|` OR, guards, `_` catch-all |

---

## Exercises

```bash
cd exercises
make run    # compile + run all exercises
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_fizzbuzz.rs` | Classic FizzBuzz 1..100 | for loop, if/else, modulo |
| 02 | `ex02_guessing.rs` | Guess the number (warmer/colder) | loop, match, if/else, input |
| 03 | `ex03_multiplication_table.rs` | Print N×N grid | nested for loops, formatting |
| 🏆 | `ex04_bonus_pi_series.rs` | Approximate π via Leibniz series | loop, break on precision, f64 math |

---

## Project — `guessing_game`

Number guessing game with a randomly generated secret number.

```bash
cd project
echo -e "50\n75\n62\n68\n" | cargo run --    # (will tell you stats at the end)
```

**Features:**
- Secret number generated from `SystemTime` nanosecond entropy
- Loop until correct guess
- "Too high" / "Too low" feedback
- Attempt tracking with stats at the end

---

## What's next?

Group 04 `04_ownership`: Ownership, moves, clones, slices — Rust's most important concept.
