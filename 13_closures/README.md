# 13 Closures — Anonymous functions that capture their environment

> Closures are like functions you can write inline, but they can also "capture" variables from the surrounding scope. Rust closures are zero-cost — they compile to the same code as hand-written functions.

---

## Cheatsheet

```rust
// Basic closure: |args| body
let add = |a, b| a + b;
println!("{}", add(2, 3));  // 5

// Capturing by reference (default)
let x = 10;
let print_x = || println!("{}", x);
print_x();  // still can use x

// Capturing by mutable reference
let mut count = 0;
let mut inc = || count += 1;
inc();  // count is now 1

// Capturing by value (move)
let s = String::from("hello");
let consume = move || println!("{}", s);
// s is moved into the closure, can't use s anymore

// Closure traits: Fn, FnMut, FnOnce
let f: fn(i32) -> i32 = |x| x * 2;  // Fn — can be called multiple times, no mutation

// Iterators with closures
vec![1, 2, 3].iter()
    .map(|x| x * 2)
    .filter(|x| x > &3)
    .collect::<Vec<_>>();
```

---

## Concepts (in order, compile and run each)

```bash
cd 13_closures
rustc --edition 2021 01_closure_basics.rs -o /tmp/cb && /tmp/cb
rustc --edition 2021 02_capture.rs -o /tmp/cap && /tmp/cap
rustc --edition 2021 03_fn_traits.rs -o /tmp/ft && /tmp/ft
rustc --edition 2021 04_move_keyword.rs -o /tmp/mv && /tmp/mv
rustc --edition 2021 05_iterator_adapters.rs -o /tmp/ia && /tmp/ia
rustc --edition 2021 06_iterator_consumers.rs -o /tmp/ic && /tmp/ic
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_closure_basics.rs` | Closure syntax | `\|args\| body`, type inference, calling |
| 02 | `02_capture.rs` | Capturing modes | By ref, by mut ref, by value |
| 03 | `03_fn_traits.rs` | Fn/FnMut/FnOnce | What each allows, compiler picks which |
| 04 | `04_move_keyword.rs` | `move` keyword | Forces ownership, needed for threads |
| 05 | `05_iterator_adapters.rs` | Iterator adapters | map, filter, take, skip, chain, zip |
| 06 | `06_iterator_consumers.rs` | Iterator consumers | collect, for_each, fold, reduce, any, all, find |

---

## Exercises

```bash
cd 13_closures/exercises
make run
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_filter_map.rs` | Read numbers, filter evens, map | Iterator chain, stdin |
| 02 | `ex02_csv_stats.rs` | Per-column CSV stats | Split, closures per column |
| 03 | `ex03_word_pipeline.rs` | Text pipeline: lowercase→split→filter→count | Closure composition |
| 🏆 | `ex04_bonus_custom_iterator.rs` | Custom Iterator for Fibonacci | Implementing Iterator trait |

---

## Project — Pipeline Filter

Read stdin, apply pipeline of transforms. Chain transforms using closure composition. Each transform is a `Box<dyn Fn(String) -> String>`.

```bash
cd 13_closures/project
echo "Hello World" | cargo run -- --upper --reverse
echo "Secret Message" | cargo run -- --rot13
echo "Hello" | cargo run -- --caesar 3
```

When you pipe text through flags and see it transformed, think:
**"Each transform is a closure stored in a Vec — I composed them at runtime."**

---
