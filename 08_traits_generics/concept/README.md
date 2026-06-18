# 08. Traits & Generics — `<T>`, Traits, Trait Bounds, Dynamic Dispatch

## One Question

**How do you write code that works with *any* type?**

Generics let you write one function that works on many types (monomorphization — each concrete type gets its own optimized copy at compile time). Traits let you specify *what capabilities* a type must have. Together they are the foundation of reusable, type-safe Rust code.

---

## Quick Start

```rust
// A generic function — works with any type T that can be compared and copied
fn largest<T: PartialOrd + Copy>(slice: &[T]) -> Option<T> {
    if slice.is_empty() { return None; }
    let mut biggest = slice[0];
    for &item in slice.iter() {
        if item > biggest { biggest = item; }
    }
    Some(biggest)
}

// A trait — defines shared behaviour
trait Area {
    fn area(&self) -> f64;
}

// Implement it for a concrete type
struct Circle { radius: f64 }
impl Area for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}
```

```
$ rustc --edition 2021 01_generic_fn.rs && ./01_generic_fn
$ make check          # compile-check everything
$ make                # compile + run everything
```

---

## Learning Path

Work through the concept files in order — each one builds on the previous:

1. **`01_generic_fn.rs`** — Generic functions. `<T>`, type parameter syntax, bounds with `PartialOrd + Copy`, the `largest<T>` pattern. Understand monomorphization: one generic definition → many concrete machine-code copies.

2. **`02_generic_struct.rs`** — Generic structs. `Point<T>`, `Pair<T, U>`, `impl<T>` blocks. Structs that hold any type (or two different types).

3. **`03_trait_basics.rs`** — Defining and implementing traits. The `Summary` trait pattern, implementing for custom types, `fmt::Display` for user-facing output.

4. **`04_trait_default.rs`** — Default method implementations. The template method pattern: traits with default bodies that call other trait methods.

5. **`05_trait_bounds.rs`** — Where clauses. `T: Trait1 + Trait2`, multiple bounds, `where` syntax for readability. Higher-ranked trait bounds (HRTB): `for<'a> T: Fn(&'a str) -> &'a str`.

6. **`06_derive_traits.rs`** — Derive macros. `#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]`. When to derive vs. implement manually. The auto-derive "ladder" for correctness.

7. **`07_trait_objects.rs`** — Trait objects. `Box<dyn Trait>`, dynamic dispatch (vtable), object safety rules. The `Draw` example with `Vec<Box<dyn Draw>>`.

8. **`08_impl_trait.rs`** — `impl Trait` syntax. Argument position (universal quantification), return position (existential types / opaque types). Closures as `impl Fn(i32) -> i32`.

9. **`09_associated_types.rs`** — Associated types. `type Item;` in traits, `Iterator` pattern with `next() -> Option<Self::Item>`.

10. **`10_operator_overload.rs`** — Operator overloading. `Add`, `Display`, `From`, `Neg`, `Index` from `std::ops`. The `Complex` number example with full arithmetic support.

---

## Common Patterns

| Pattern | Idiom |
|---------|-------|
| Generic function | `fn foo<T: Bound>(x: T)` |
| Generic struct | `struct Point<T> { x: T, y: T }` |
| Trait definition | `trait MyTrait { fn method(&self) -> Ret; }` |
| Trait implementation | `impl MyTrait for MyType { ... }` |
| Multiple trait bounds | `fn bar<T: Clone + Debug>(x: T)` |
| Where clause | `fn baz<T>(x: T) where T: Clone + Debug` |
| Derive macro | `#[derive(Debug, Clone, PartialEq)]` |
| Trait object (dynamic) | `Box<dyn Trait>` / `&dyn Trait` |
| `impl Trait` (argument) | `fn foo(x: impl Display)` |
| `impl Trait` (return) | `fn foo() -> impl Display` |
| Associated type | `trait Iter { type Item; fn next(&mut self) -> Option<Self::Item>; }` |
| Operator overload | `impl Add for MyType { type Output = ...; fn add(self, rhs: Self) -> Self::Output; }` |
| Supertrait | `trait PrettyPrint: Debug { ... }` |
| Default method | `fn method(&self) -> Type { default_impl() }` |
| Blanket impl | `impl<T: Debug> PrettyPrint for T {}` |

---

## Now Build Your Own

Work through the exercises in order:

1. **`ex01_stats.rs`** — Generic stats. Implement `mean`, `median`, `mode` on `&[T]`. Uses `Into<f64>`, `PartialOrd`, `Eq + Hash` bounds. Works with both integers and floats.

2. **`ex02_to_any_base.rs`** — The `ToBase` trait. Convert numbers to any base string (2–36). Build a trait with a required method + convenience defaults (`to_binary`, `to_hex`). Handle negative numbers for signed types.

3. **`ex03_shape_area.rs`** — The `Area` trait for shapes. `Circle`, `Square`, `Triangle` implement `area() -> f64`. Generic `Rectangle<T, U>` where dimensions implement `Into<f64>`. `total_area` via `&[impl Area]`. Mixed collection via `Vec<Box<dyn Area>>`.

4. **BONUS: `ex04_pretty_print.rs`** — The `PrettyPrint` trait with Debug supertrait, default impl, and custom overrides. Demonstrates the key insight: **Rust does not allow overriding default methods from blanket impls** — each type must opt in explicitly.

Then build the **`calculator`** project:

```
cd projects/calculator
cargo run -- add 5 3            # 8
cargo run -- sub 10.5 2.5       # 8
cargo run -- mul "3+4i" "1+2i"  # -5+10i
cargo run -- div 100 7          # 14
cargo run -- --help             # show usage
```

The calculator uses a shared `Calculator` trait (with `Display` + `Sized` supertraits, `parse`, `add`, `sub`, `mul`, `div` methods) implemented for `i32`, `f64`, and `Complex`. A generic `calculate<T: Calculator>` engine dispatches based on input format detection.
