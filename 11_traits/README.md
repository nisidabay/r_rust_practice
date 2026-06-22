# 11 Traits — Defining shared behavior

> Traits are Rust's version of interfaces — but better. They define shared behavior without inheritance. You can implement a trait for ANY type, even built-in ones.

---

## Cheatsheet

```rust
// --- Define a trait ---
trait Summary {
    fn summarize(&self) -> String;
}

// --- Implement for a type ---
struct Article { title: String, author: String }
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("\"{}\" by {}", self.title, self.author)
    }
}

// --- Default implementations ---
trait Greeter {
    fn greet(&self) -> String { "Hello!".to_string() }  // default
    fn name(&self) -> String;                            // must override
}

// --- Derive common traits ---
#[derive(Debug, Clone, PartialEq)]
struct User { id: u64, name: String }

// --- Trait bounds (generics) ---
fn print_summary<T: Summary>(item: T) { /* ... */ }
fn notify<T>(item: T) where T: Summary + Display { /* ... */ }

// --- impl Trait syntax ---
fn print_it(item: impl Display) { /* ... */ }
fn make_greeter() -> impl Display { "hello" }

// --- Trait objects (dynamic dispatch) ---
fn print_animal(a: &dyn Animal) { /* ... */ }
let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog{}), Box::new(Cat{})];

// --- Operator overloading ---
impl Add for Complex { type Output = Complex; /* ... */ }
// Now: c1 + c2 works!
```

---

## Concepts (in order, compile and run each)

```bash
cd 11_traits
rustc --edition 2021 concept/01_trait_define.rs -o /tmp/t1 && /tmp/t1
rustc --edition 2021 concept/02_trait_default.rs -o /tmp/t2 && /tmp/t2
rustc --edition 2021 concept/03_derive.rs -o /tmp/t3 && /tmp/t3
rustc --edition 2021 concept/04_trait_bounds.rs -o /tmp/t4 && /tmp/t4
rustc --edition 2021 concept/05_impl_trait.rs -o /tmp/t5 && /tmp/t5
rustc --edition 2021 concept/06_trait_objects.rs -o /tmp/t6 && /tmp/t6
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_trait_define.rs` | Defining traits | `trait`, `impl Trait for Type`, methods | 
| 02 | `02_trait_default.rs` | Default implementations | Default method bodies, optional override | 
| 03 | `03_derive.rs` | #[derive(...)] | Debug, Clone, PartialEq, Copy, Eq, Hash, Default |
| 04 | `04_trait_bounds.rs` | Generic constraints | `<T: Trait>`, `where` clause, multiple bounds, impl Trait return |
| 05 | `05_impl_trait.rs` | impl Trait syntax | Anonymous generics, opaque return types, closures |
| 06 | `06_trait_objects.rs` | Dynamic dispatch | `&dyn Trait`, `Box<dyn Trait>`, vtable, heterogeneous collections |

---

## Exercises

```bash
cd 11_traits/exercises
make run
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_area.rs` | Area trait for Rectangle, Circle, Triangle, sum in Vec | Trait definition, impl, Box\<dyn Trait\> |
| 02 | `ex02_to_base.rs` | ToBase trait, convert u32 to any base 2-36 | impl for built-in, custom trait |
| 03 | `ex03_pretty.rs` | PrettyPrint trait for Vec\<T\> and Option\<T\> | Generic impl, trait bounds |
| 🏆 | `ex04_bonus_complex.rs` | Complex with Add and Display | Operator overloading, Add, Display, Debug, Clone, PartialEq |

---

## Project — pretty_printer

Read structured data like `"name: Alice, age: 30, scores: [85,92,78]"`. Parse and pretty-print with ANSI colors. Supports `--json` flag. Uses traits for different output formats.

```bash
cd 11_traits/project
cargo run -- "name: Alice, age: 30, scores: [85,92,78]"
cargo run -- "name: Bob, age: 25, scores: [90,88,95]" --json
echo "name: Carol, age: 28, scores: [70,80]" | cargo run --
```

When you see the colored table or JSON output, think:
**"Each format is a trait implementation. Adding a new format just means impl FormatOutput for MyFormat."**

---

## What's next?

Group 12 `12_lifetimes`: Rust's reference-checking system — how the borrow checker knows references are valid.
