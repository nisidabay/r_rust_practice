# 02 Data — Numbers, booleans, characters, conversions, constants, and formatting

> Learn how Rust represents and transforms data. 6 concepts + 4 exercises + 1 project.

---

## Cheatsheet

```rust
// How to work with numbers
let x: i32 = 42;                              // signed 32-bit (default int)
let y: u64 = 100;                             // unsigned 64-bit
let z: f64 = 3.14;                            // double-precision float
let overflow = 255u8.wrapping_add(1);          // wraps to 0 instead of panicking
let checked = 255u8.checked_add(1);            // returns None instead of panicking

// How to use booleans
let is_big = x > 30;                          // comparison produces bool
let both = is_big && (y > 50);                // && short-circuits
let either = is_big || (y > 50);              // || short-circuits

// How to use characters
let c = 'R';                                  // 4-byte Unicode
c.is_alphabetic();                            // true for Unicode letters
c.to_uppercase().to_string();                 // Unicode-aware case conversion

// How to convert between types
let from_f64 = 3.99 as i32;                   // truncates to 3
let from_str: f64 = "3.14".parse().unwrap();  // parse a string
let from_into: i64 = 42i32.into();            // Into trait (safe widening)

// How to declare constants
const MAX: u32 = 100;                         // compile-time, inlined
const fn square(x: i32) -> i32 { x * x }      // runs at compile time

// How to format output
println!("{:b} {:x} {:o}", 255, 255, 255);    // binary, hex, octal
println!("{:>10} {:<10}", "right", "left");   // alignment
println!("{:.2}", std::f64::consts::PI);      // precision (3.14)
```

---

## Concepts (in order, compile and run each)

```bash
rustc --edition 2021 concept/01_numbers.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/02_booleans.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/03_char.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/04_type_conversion.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/05_constants.rs -o /tmp/ex && /tmp/ex
rustc --edition 2021 concept/06_formatting.rs -o /tmp/ex && /tmp/ex
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_numbers.rs` | Integer and float types | i32, u64, f64, arithmetic, overflow (wrap vs panic), `as` casting |
| 02 | `02_booleans.rs` | Boolean logic | `bool`, `== != < >`, `&& || !`, short-circuit evaluation |
| 03 | `03_char.rs` | Unicode characters | 4-byte char, `is_alphabetic`, `is_digit`, `to_uppercase` |
| 04 | `04_type_conversion.rs` | Type conversions | `as` casting, `From`/`Into` traits, `parse()` with turbofish `::` |
| 05 | `05_constants.rs` | Compile-time constants | `const` vs `let`, `const fn`, compile-time evaluation |
| 06 | `06_formatting.rs` | Formatted output | `{:b}` `{:x}` `{:o}` `{:e}`, padding, alignment, precision |

---

## Exercises

```bash
cd exercises
make run    # compile + run all exercises
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_unit_convert.rs` | Convert between units (length) | input, parse, f64 arithmetic, match |
| 02 | `ex02_temperature_table.rs` | Print °C → °F conversion table | loops, formatting, padding, f64 math |
| 03 | `ex03_compound_interest.rs` | Yearly compound interest table | input, loops, f64 pow, formatting precision |
| 🏆 | `ex04_bonus_primes.rs` | Find all primes up to N | trial division, sqrt optimization, loops |

---

## Project — `unit_converter`

CLI converter between units of length, weight, and time.

```bash
cd project
echo "150 cm to m" | cargo run --    # → 150 cm = 1.5 m
echo "5 kg to g"   | cargo run --    # → 5 kg = 5000 g
echo "2 h to min"  | cargo run --    # → 2 h = 120 min
```

**Supported units:**
- Length: mm, cm, m, km, in, ft
- Weight: mg, g, kg, lb
- Time: s, min, h, day

Reads lines from stdin, parses `"<value> <from_unit> to <to_unit>"`, converts via a base unit (m, g, s), and prints the result.

---

## What's next?

Group 03 `03_control`: if/else, loops, match — Rust's control flow.
