# 01 Hello — How to print, read, and do things

> Learn Rust as a tool, not a religion. 6 concepts + 1 project. That's it.

---

## Cheatsheet

```rust
// How to print
println!("Hello {}", name);       // with newline
print!("No newline");              // without newline

// How to read from keyboard
let mut buf = String::new();
io::stdin().read_line(&mut buf).unwrap();
let trimmed = buf.trim();          // remove trailing \n

// How to read a number
let n: i32 = input.trim().parse().expect("Not a number");

// How to work with text
let s = String::from("hello");
s.len();                           // length in bytes
s.trim();                          // remove whitespace
s.contains("ell");                 // check substring
s.split_whitespace()               // split by whitespace
format!("{} + {} = {}", 3, 5, 8);  // build string (like sprintf)

// How to store multiple values
let arr = [10, 20, 30];            // fixed-size array
let mut v = vec![1, 2, 3];         // growable Vec
v.push(4);
v.pop();                           // remove last
```

That's it. You can print, read input, handle text, and store lists. Everything else is practice.

---

## Concepts (in order, compile and run each)

```bash
# Try each concept
rustc --edition 2021 concept/01_hello.rs -o /tmp/hello && /tmp/hello
rustc --edition 2021 concept/02_variables.rs -o /tmp/vars && /tmp/vars
rustc --edition 2021 concept/03_types.rs -o /tmp/types && /tmp/types
rustc --edition 2021 concept/04_input.rs -o /tmp/input && echo "Carlos" | /tmp/input
rustc --edition 2021 concept/05_strings.rs -o /tmp/strings && /tmp/strings
rustc --edition 2021 concept/06_arrays.rs -o /tmp/arrays && /tmp/arrays
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_hello.rs` | Print with `println!` | `{}` placeholders, `{:?}` debug, macros |
| 02 | `02_variables.rs` | Create and change variables | `let` vs `let mut`, `const`, shadowing |
| 03 | `03_types.rs` | Choose the right type | `i32`, `f64`, `bool`, `char` (Unicode!), tuples, arrays |
| 04 | `04_input.rs` | Read from keyboard | `read_line`, `.trim()`, `.parse()`, loop until EOF |
| 05 | `05_strings.rs` | Handle text | `String` vs `&str`, `.push_str`, `.contains`, `.split_whitespace`, `format!` |
| 06 | `06_arrays.rs` | Store lists | `[T; N]` arrays, `Vec<T>`, `.push`, `.pop`, `.iter().sum()` |

---

## Exercises

```bash
cd exercises
make check    # compile + run all exercises
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_fahrenheit.rs` | Celsius → Fahrenheit converter | input, parse, f64 arithmetic |
| 02 | `ex02_line_stats.rs` | Per-line char/word counter | BufRead, split_whitespace, enumerate |
| 03 | `ex03_sum_numbers.rs` | Sum + average from stdin | loop, accumulate, float division |
| 🏆 | `ex04_bonus_collatz.rs` | Print Collatz sequence | while loop, u64 arithmetic, conditionals |

---

## Project — `wc`

Counts lines, words, and characters from stdin. **Exactly like the C version.**

```bash
cd project
echo "hello world" | cargo run --    # → 1 lines, 2 words, 11 chars
cat src/main.rs | cargo run --       # counts its own source
```

When you run `wc` and see the output, you should think:
**"I built my own wc with 6 building blocks. Now everything makes sense."**

---

## What's next?

Group 02 `02_data`: Numbers, strings, booleans, and how to convert between them — deeper.
