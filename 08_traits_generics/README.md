# 08 — Traits & Generics

**One question answered:** *How do you write code that works with **any** type?*

## Concepts

| File | Topic | Key Ideas |
|------|-------|-----------|
| `01_generic_fn.rs` | Generic functions | `<T>`, type parameters, monomorphization |
| `02_generic_struct.rs` | Generic structs | `Point<T>`, `Pair<T,U>`, `impl<T>` |
| `03_trait_basics.rs` | Traits | Defining, implementing, `Display` |
| `04_trait_default.rs` | Default methods | Default impl bodies, template method pattern |
| `05_trait_bounds.rs` | Where clauses | `T: Trait1 + Trait2`, HRTB |
| `06_derive_traits.rs` | Derive macros | `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` |
| `07_trait_objects.rs` | Trait objects | `Box<dyn Trait>`, dynamic dispatch, object safety |
| `08_impl_trait.rs` | `impl Trait` | Argument/return position, opaque types, closures |
| `09_associated_types.rs` | Associated types | `type Item;`, Iterator pattern |
| `10_operator_overload.rs` | Operator overloading | `Add`, `Display`, `From`, `Neg`, `Index` |

## Exercises

| Exercise | Description |
|----------|-------------|
| `ex01_stats.rs` | Generic stats — mean, median, mode on `&[T]` |
| `ex02_to_any_base.rs` | `ToBase` trait — convert numbers to any base string |
| `ex03_shape_area.rs` | `Area` trait — Circle, Square, Triangle + generic `Rectangle<T,U>` |
| `ex04_pretty_print.rs` (BONUS) | `PrettyPrint` trait with blanket impl + custom overrides |

## Project: `calculator`

A generic calculator supporting `i32`, `f64`, and `Complex` numbers via a shared `Calculator` trait.

```bash
cd projects/calculator
cargo run -- add 5 3
cargo run -- sub 10.5 2.5
cargo run -- mul "3+4i" "1+2i"
cargo run -- div 100 7
cargo run -- help
```

## Run Everything

```bash
make          # build run all concepts, exercises, and the project
make check    # compile-check everything (no execution)
make clean    # remove binaries and target dirs
```
