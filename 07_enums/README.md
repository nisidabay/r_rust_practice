# 07 Enums — Custom types with distinct variants

> Rust enums are NOT integer constants (like C). They're full types that can carry data. Each variant is a distinct value, and the compiler checks every match is exhaustive.

---

## Cheatsheet

```rust
// Define an enum — each variant is a value, not an int
enum Direction { North, South, East, West }

// Variants with data — tagged union, type-safe
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Match and extract — exhaustive, must handle ALL variants
match msg {
    Message::Quit => println!("bye"),
    Message::Move { x, y } => println!("({}, {})", x, y),
    Message::Write(t) => println!("{}", t),
    Message::ChangeColor(r, g, b) => {}
}

// Option<T> — nullable value, type-safe (no NULL pointers)
let x: Option<i32> = Some(42);
let y: Option<i32> = None;
x.is_some();                     // true
x.is_none();                     // false
x.unwrap();                      // 42 — panics if None
x.unwrap_or(0);                  // 42 (or 0 if None)
x.map(|v| v * 2);                // Some(84)
if let Some(v) = x { /* use v */ }

// Result<T, E> — fallible operation
let r: Result<i32, &str> = Ok(42);
let e: Result<i32, &str> = Err("fail");
r.is_ok();                        // true
r.is_err();                       // false
r.unwrap_or_else(|e| { 0 });      // handle error with fallback
r.map(|v| v * 2);                 // Ok(84)
r.map_err(|e| format!("ERR: {}", e));

// if let — match one variant concisely
if let Some(v) = opt { /* v is unwrapped */ }
while let Some(v) = iter.next() { /* loop until None */ }

// impl on enums — methods + associated constants
enum Temp { C(f64), F(f64) }
impl Temp {
    const ABSOLUTE_ZERO: f64 = -273.15;
    fn to_celsius(&self) -> f64 { /* ... */ }
}
```

---

## Concepts (in order, compile and run each)

```bash
# Try each concept
rustc --edition 2021 concept/01_enum_basics.rs -o /tmp/check && /tmp/check
rustc --edition 2021 concept/02_enum_data.rs -o /tmp/check && /tmp/check
rustc --edition 2021 concept/03_option.rs -o /tmp/check && /tmp/check
rustc --edition 2021 concept/04_result.rs -o /tmp/check && /tmp/check
rustc --edition 2021 concept/05_if_let.rs -o /tmp/check && /tmp/check
rustc --edition 2021 concept/06_enum_impl.rs -o /tmp/check && /tmp/check
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_enum_basics.rs` | Define enum, match on variants | Each variant is a value, match is exhaustive |
| 02 | `02_enum_data.rs` | Variants with data (named, unnamed) | Tagged unions — extracted via pattern matching |
| 03 | `03_option.rs` | Option&lt;T&gt; — Some and None | No NULL pointers; match, unwrap, unwrap_or, map |
| 04 | `04_result.rs` | Result&lt;T,E&gt; — Ok and Err | No exceptions; is_ok, unwrap_or_else, map_err |
| 05 | `05_if_let.rs` | if let and while let | Concise single-variant matching |
| 06 | `06_enum_impl.rs` | impl on enums with methods | Methods + associated constants on enums |

---

## Exercises

```bash
cd exercises
make run    # compile + run all exercises
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_http_status.rs` | Match HTTP status codes to descriptions | Enum variants, match |
| 02 | `ex02_color_hex.rs` | Parse "#FF8800" into RGB | Option, pattern matching |
| 03 | `ex03_calculator.rs` | Read "3 + 4", parse, compute | Enum with data, stdin, match |
| 🏆 | `ex04_bonus_command.rs` | REPL with Quit/Help/Add/List | Enum dispatch, loop, Vec |

---

## Project — CLI Dispatcher

Reads commands like `add hello`, `list`, `done 2`, `quit` from stdin.
Uses an enum (Command) for clean dispatch — the compiler ensures every command is handled.

```bash
cd project
echo -e "add buy milk\nadd walk dog\nlist\ndone 2\nquit" | cargo run
```

When you type commands and see them dispatched through an enum, you should think:
**"Enums plus match = the cleanest command dispatcher I've ever written."**

---

## What's next?

Group 08 `08_collections`: Vec, HashMap, HashSet, iterators — managing groups of values.
