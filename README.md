# Practical Rust

> Learn Rust as tool, not as religion. **Code first. Theory on demand.**

```bash
git clone git@github.com:nisidabay/r_rust_practice.git
cd r_rust_practice
rustc --edition 2021 01_hello/concept/01_hello.rs -o /tmp/hello && /tmp/hello
```

---

## Philosophy

Every file answers one question: **"How do I do X in Rust?"**

Not "here's everything about X." Not "first, let me explain generics vs trait objects." Just code that works, a short explanation, and a project that ties it all together.

If you want the full theory, the books are listed in `REFERENCES.md`. This course is the **fast path** — the 20% of Rust that does 80% of the work.

The structure is the same as [Practical C](https://github.com/nisidabay/curso_c). Same questions, same project types — different language. Learn one, and you already know the path through the others.

---

## The Path (16 groups)

| # | Group | One Question | Project |
|---|-------|-------------|---------|
| 01 | `01_hello/` | How do I print, read, and store things? | `wc` clone |
| 02 | `02_data/` | What kinds of data can I store and convert? | Unit converter |
| 03 | `03_control/` | How does Rust decide what runs next? | Number guessing game |
| 04 | `04_ownership/` | Who owns the data — and what happens when they're done? | String sanitizer |
| 05 | `05_references/` | How do I borrow data without stealing it? | JSON field extractor |
| 06 | `06_structs/` | How do I bundle related data together? | Student gradebook |
| 07 | `07_enums/` | How do I model "one of several possibilities"? | CLI dispatcher |
| 08 | `08_collections/` | What do I reach for when I need more than one value? | Frequency counter |
| 09 | `09_patterns/` | How do I safely unpack any shape of data? | Mini grep |
| 10 | `10_errors/` | How do I handle things that can go wrong — without panicking? | File reader |
| 11 | `11_traits/` | How do I define shared behavior across types? | Pretty printer |
| 12 | `12_lifetimes/` | How long does each reference live — and who decides? | HTML builder |
| 13 | `13_closures/` | How do I pass behavior around? | Pipeline filter |
| 14 | `14_io/` | How does Rust talk to files and the outside world? | Log line counter |
| 15 | `15_testing/` | How do I know my code works — and stays working? | Calculator with tests |
| 16 | `16_capstone/` | How do all these pieces fit together? | Todo list manager |

---

## Quick Start

Each group is self-contained. Start at 01 and work your way through:

```bash
# Run any concept file
rustc --edition 2021 01_hello/concept/01_hello.rs -o /tmp/demo && /tmp/demo

# Try the exercises
cd 01_hello/exercises && make run

# Build the project
cd 01_hello/project && cargo run --
```

---

## Structure

```
NN_topic/
├── concept/        # Numbered .rs files — one concept at a time
├── exercises/      # Solved practice problems with Makefile
│   └── Makefile    # make run compiles + runs everything
└── project/        # One self-contained CLI mini-app (Cargo)
    ├── Cargo.toml
    └── src/main.rs
```

---

## Resources

Book recommendations in [`REFERENCES.md`](REFERENCES.md).

---

## License

Do whatever you want with this. It's a learning resource.
