# Rust Practice — From Zero to Real Programs

> The one question: **How does Rust give you C-level control with memory safety?**

A progressive, code-first curriculum for learning Rust through CLI programs.
No frameworks, no `cargo add` — just the standard library, `rustc`, and real problems.

```bash
# Clone, compile, learn
git clone git@github.com:nisidabay/r_rust_practice.git
cd r_rust_practice
rustc --edition 2021 01_hello_types/concept/01_hello.rs -o /tmp/hello && /tmp/hello
```

## The Learning Path

| # | Group | One Question | Concepts | Exercises | Project |
|---|-------|-------------|----------|-----------|---------|
| 01 | `01_hello_types/` | What is a Rust program — and what lives in memory? | — | — | — |
| 02 | `02_control_flow/` | How does Rust decide what runs next? | — | — | — |
| 03 | `03_ownership/` | Who owns the data — and what happens when they're done? | — | — | — |
| 04 | `04_structs_enums/` | How do you shape your own data types? | — | — | — |
| 05 | `05_collections/` | What do you reach for when you need more than one value? | — | — | — |
| 06 | `06_pattern_matching/` | How do you safely unpack any shape of data? | — | — | — |
| 07 | `07_error_handling/` | How do you handle things that can go wrong — without panicking? | — | — | — |
| 08 | `08_traits_generics/` | How do you write code that works with *any* type? | — | — | — |
| 09 | `09_lifetimes/` | How long does each reference live — and who decides? | — | — | — |
| 10 | `10_closures_iterators/` | How do you pass behavior around — cheaply and safely? | — | — | — |
| 11 | `11_modules_crates/` | How do you grow your program without drowning in one file? | — | — | — |
| 12 | `12_testing/` | How do you know your code works — and stays working? | — | — | — |
| 13 | `13_io_filesystem/` | How does Rust talk to the outside world? | — | — | — |
| 14 | `14_smart_pointers/` | What happens when ownership rules are too strict? | — | — | — |
| 15 | `15_concurrency/` | How do you run code in parallel — safely? | — | — | — |
| 16 | `16_unsafe_ffi/` | What do you do when you need to break the rules? | — | — | — |
| 17 | `17_macros/` | How do you write code that writes code? | — | — | — |
| 18 | `18_async/` | How do you handle many things at once — without threads? | — | — | — |

## How to Use This Repo

Each group is self-contained. Start at 01 and work forward — every group builds on the last.

```bash
# Explore a group
cd 03_ownership

# Run a concept file
rustc --edition 2021 concept/01_ownership.rs -o /tmp/ownership && /tmp/ownership

# Try the exercises
cd exercises && make check

# Build the project
cd project && cargo run -- --help
```

## Prerequisites

```bash
rustc --version   # Should be ≥ 1.70 (edition 2021 support)
cargo --version   # Comes with Rust — install via rustup
```

Install from https://rustup.rs/ or:
```bash
# Arch Linux
sudo pacman -S rust
```

## Structure

```
NN_topic/
├── concept/        # Numbered .rs files — one concept at a time
│   ├── 01_hello.rs
│   └── 02_variables.rs
├── exercises/      # Solved practice problems with Makefile
│   ├── ex01_title.rs
│   ├── ex02_title.rs
│   └── Makefile
└── project/        # One self-contained CLI mini-app
    ├── Cargo.toml
    └── src/main.rs
```
