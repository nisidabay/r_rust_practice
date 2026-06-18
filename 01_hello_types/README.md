# 01 — hello_types

## One Question

**What is a Rust program — and what lives in memory?**

Every Rust program starts with `fn main()` — the single entry point. Inside it, you bind names to values: integers, floats, booleans, characters, tuples, arrays. Each binding occupies a slot in memory (stack for now), and the type decides both the bit pattern and what operations are allowed. Before you can build anything meaningful, you need to know: what shapes can data take, and how do you create, name, and transform them?

## Quick Start

```bash
# Concept files — compile and run each standalone
cd concept
rustc --edition 2021 01_hello.rs -o /tmp/hello && /tmp/hello
rustc --edition 2021 02_variables.rs -o /tmp/vars && /tmp/vars
# ... try all six concept files

# Exercises
cd exercises
make check          # compile + run all exercises
make clean          # remove binaries

# Project — CLI counter
cd project/counter
cargo run -- --help
cargo run -- --count 12 --reverse --format hex
```

## Learning Path

| Step | File | What You'll Learn | Key Concept |
|------|------|-------------------|-------------|
| 1 | `concept/01_hello.rs` | Program structure, `fn main()`, `println!` | Entry point, macros |
| 2 | `concept/02_variables.rs` | `let`, `mut`, `const`, shadowing | Immutability by default |
| 3 | `concept/03_scalar_types.rs` | `i32`, `u64`, `f64`, `bool`, `char` | Scalar type system |
| 4 | `concept/04_type_inference.rs` | How Rust infers types from context | Local inference |
| 5 | `concept/05_tuple_array.rs` | Tuple and array syntax, indexing, destructuring | Compound types |
| 6 | `concept/06_range.rs` | `..` and `..=` ranges, `.rev()`, `.step_by()` | Iteration primitives |
| 7 | `exercises/ex01_temperature.rs` | CLI args, `f64` arithmetic, format specifiers | Real I/O |
| 8 | `exercises/ex02_fibonacci.rs` | Tuple swap, `u64`, loop counters | State mutation |
| 9 | `exercises/ex03_twelve_days.rs` | Parallel arrays, nested range loops | Cumulative output |
| 🏆 | `exercises/ex04_bonus_pi.rs` | Leibniz series, `step_by`, `f64` accumulation | Numerical methods |
| 🚀 | `project/counter/` | Full CLI tool with `--count`, `--step`, `--reverse`, `--format` | Cargo project |

## Common Patterns

| Pattern | Example | Why |
|---------|---------|-----|
| Default immutability | `let x = 5;` | Prevents accidental mutation; `mut` is explicit opt-in |
| Shadowing for type change | `let x = x + 1; let x = "str";` | Reuse names without fighting the type system |
| Tuple destructuring | `let (a, b) = (b, a + b);` | Single-statement state swap |
| Range iteration | `for i in 0..10` | Half-open ranges are the Rust idiom |
| Inclusive range + rev | `(0..=n).rev()` | Count down including the start |
| Parse with turbofish | `s.parse::<f64>()` | Disambiguates `parse` when inference isn't enough |
| `as` cast | `denom as f64` | Explicit numeric conversion (no implicit widening) |
| `{val:#x}` format | `println!("{val:#x}")` | Hex output with `0x` prefix |

## Now Build Your Own

Try these without looking at the solutions:

1. **Wind Chill** — Take temperature (°C) and wind speed (km/h); print wind chill using the formula:  
   `W = 13.12 + 0.6215*T - 11.37*V^0.16 + 0.3965*T*V^0.16`

2. **Collatz Sequence** — Print the Collatz sequence starting from a given N. Stop when you reach 1. Count the steps.

3. **Sieve of Eratosthenes** — Use a `bool` array and ranges to print all primes up to some limit N.

4. **Roman Numeral Printer** — Take a number 1..3999 and print it as Roman numerals using arrays for lookup tables.
