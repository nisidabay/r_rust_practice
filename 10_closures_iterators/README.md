# 10 — closures_iterators

## One Question

**How do you pass behavior around — cheaply and safely?**

## Quick Start

```bash
# Concept files — compile and run each standalone
cd concept
rustc --edition 2021 01_closure_basics.rs -o /tmp/c01 && /tmp/c01
rustc --edition 2021 02_closure_capture.rs -o /tmp/c02 && /tmp/c02
# ... try all nine concept files

# Exercises
cd exercises
make check          # compile + run all exercises
make clean          # remove binaries

# Project — log analyzer CLI
cd project
cargo run -- sample.log --level ERROR --top 5
cargo run -- sample.log --level WARN
cargo run -- --help
```

## Learning Path

| Step | File | What You'll Learn | Key Concept |
|------|------|-------------------|-------------|
| 1 | `concept/01_closure_basics.rs` | `\|args\| body` syntax, type inference | Anonymous functions |
| 2 | `concept/02_closure_capture.rs` | By reference, by value, `move` closures | Capture modes |
| 3 | `concept/03_fn_fnt_mut.rs` | `Fn`, `FnMut`, `FnOnce` traits | Closure trait hierarchy |
| 4 | `concept/04_closure_as_arg.rs` | Passing closures to functions, generics | Higher-order functions |
| 5 | `concept/05_iterator_trait.rs` | `Iterator` trait, `next()`, consuming adapters | Core iteration protocol |
| 6 | `concept/06_iterator_adapters.rs` | `map`, `filter`, `take`, `skip`, `chain` | Lazy transformation |
| 7 | `concept/07_iterator_consumers.rs` | `collect`, `sum`, `count`, `fold`, `for_each` | Driving iteration |
| 8 | `concept/08_iterator_composition.rs` | Chaining multiple adapters | Declarative pipelines |
| 9 | `concept/09_owning_iterator.rs` | `into_iter()` vs `iter()` vs `iter_mut()` | Ownership in iteration |
| 10 | `exercises/ex01_filter_map.rs` | `filter_map` for combined filter+map | Parsing with fallback |
| 11 | `exercises/ex02_csv_stats.rs` | CSV parsing, `fold`, `sum`, `powi` | Statistical computation |
| 12 | `exercises/ex03_word_count_pipeline.rs` | `HashMap`, chained pipeline, `for_each` | Frequency analysis |
| 🏆 | `exercises/ex04_custom_iterator.rs` | Implementing `Iterator` for `Fibonacci`, `RangeStep` | Custom iteration |
| 🚀 | `project/log_analyzer` | Full CLI with `--level`, `--top`, iterator chains | Real tool |

## Common Patterns

| Pattern | Example | Why |
|---------|---------|-----|
| Closure syntax | `\|x, y\| x + y` | Lightweight anonymous function |
| Capture by reference (default) | `let f = \|\| println!("{x}");` | Least-permissive, most flexible |
| `move` closure | `let f = move \|\| drop(x);` | Needed when closure outlives scope (threads, returning) |
| `Fn` for read-only | `fn apply<F: Fn(i32) -> i32>(f: F)` | Callable multiple times, no mutation |
| `FnMut` for stateful | `fn apply<F: FnMut(i32) -> i32>(mut f: F)` | Callable multiple times, may mutate captures |
| `FnOnce` for consuming | `fn apply<F: FnOnce(i32) -> i32>(f: F)` | Callable once (consumes captures) |
| `iter()` for borrowing | `for x in vec.iter()` | Doesn't consume collection |
| `into_iter()` for consuming | `for x in vec` | Takes ownership, most efficient |
| `filter_map` | `.filter_map(\|s\| s.parse().ok())` | Filter + map in one pass |
| `fold` as universal consumer | `.fold(0, \|acc, x\| acc + x)` | Most general accumulation |
| Chained pipeline | `.filter(...).map(...).take(...).collect()` | Declarative data processing |
| Custom `Iterator` | `impl Iterator for MyStruct` | Extend iteration to your types |

## Now Build Your Own

Try these without looking at the solutions:

1. **Running Average** — Given a stream of numbers, compute a running average using `scan` or `fold`.

2. **Log Parser with Severity** — Parse log lines with different severity levels and filter by minimum severity (e.g., show WARN and above).

3. **Lazy Prime Generator** — Implement an iterator that lazily generates prime numbers using trial division.

4. **Collatz Iterator** — Create a `Collatz` struct that implements `Iterator`, yielding the Collatz sequence from a starting value until it reaches 1.
