# 04 — Structs & Enums

> **How do you shape your own data types?**

Rust lets you define your own data types with `struct` (product types) and `enum` (sum types). This group covers how to create, use, and extend custom types.

## Concepts

| # | File | What You'll Learn |
|---|------|-------------------|
| 01 | `01_struct_basics.rs` | Named structs, field access, mutability, update syntax, unit-like structs |
| 02 | `02_tuple_structs.rs` | Tuple structs, positional access, destructuring, newtype pattern |
| 03 | `03_struct_methods.rs` | `impl` blocks, `&self`, `&mut self`, `self`, multiple `impl` blocks |
| 04 | `04_associated_fns.rs` | Associated functions (`::new()`), `Self`, named constructors |
| 05 | `05_enum_basics.rs` | Enum definition, variants, `match`, wildcard `_` |
| 06 | `06_enum_data.rs` | Enums with data (named fields, tuple variants, unit variants) |
| 07 | `07_option_enum.rs` | `Option<T>`, `Some`, `None`, `unwrap`, `expect`, `map`, `unwrap_or` |
| 08 | `08_result_enum.rs` | `Result<T, E>`, `Ok`, `Err`, `map`, `unwrap_or_else`, error types |
| 09 | `09_if_let_while_let.rs` | `if let`, `while let` — concise pattern matching |

## Exercises

| Exercise | File | Description |
|----------|------|-------------|
| ex01 | `ex01_rectangle.rs` | Struct `Rectangle` with `area()`, `perimeter()`, `is_square()` methods |
| ex02 | `ex02_traffic_light.rs` | Enum `TrafficLight` with `duration()` returning seconds per variant |
| ex03 | `ex03_elevator.rs` | Enum `ElevatorState` (`Moving`/`Idle`) with `current_floor()` and `description()` |
| BONUS | `ex04_shapes.rs` | Enum `Shape` (Circle, Rect, Triangle) with `area()` and `perimeter()` |

Run all exercises:
```bash
cd exercises && make check
```

## Project: `task_list`

A simple CLI task manager using `enum TaskStatus` (Pending, InProgress, Done) and `struct Task` (id, title, status).

```bash
cd project
cargo run -- add "Buy groceries"
cargo run -- list
cargo run -- complete 1
```

## Key Takeaway

`struct` bundles related data together (product types). `enum` lets one value be one of several shapes (sum types). Together with `impl`, they are the foundation of Rust's type system — everything else (traits, generics, pattern matching) builds on this.
