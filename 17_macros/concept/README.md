# 17_macros — macro_rules!, repetition, designators, recursive macros

## One Question
**How do you write code that writes code?**

## Answer
Rust's **declarative macros** (`macro_rules!`) let you define code transformations using pattern matching. They expand at compile time, generating Rust code from a compact template syntax.

## Files

| File | Description |
|------|-------------|
| `01_declarative_intro.rs` | `macro_rules!` basics — patterns, arms, matching |
| `02_repetition.rs` | `$()*` / `$()+,` repetition, separators, accumulator patterns |
| `03_designators.rs` | `$expr`, `$ty`, `$ident`, `$block`, `$tt`, `$literal` and more |
| `04_recursive_macros.rs` | Recursive macros for complex token streams |
| `05_vec_macro.rs` | Re-implement `vec![]` to understand how it works |
| `06_macro_hygiene.rs` | Hygiene, `$crate`, call-site vs definition-site scoping |
| `07_build_derive.rs` | Proc-macro derive macros — `syn`/`quote` concepts (documentation only) |

## Key Concepts

1. **Pattern matching**: `macro_rules!` arms match token trees, not strings
2. **Designators**: `$x:expr` tells the parser what kind of token to expect
3. **Repetition**: `$($x:expr),*` captures zero-or-more, `+` captures one-or-more
4. **Recursion**: Macros can call themselves — base case + recursive case
5. **Hygiene**: Most identifiers in macros don't conflict with caller scope
6. **`$crate`**: Absolute path to the defining crate for library macros

## Exercises

| Exercise | Description |
|----------|-------------|
| `ex01_assert_macro` | Build a custom `assert!` equivalent |
| `ex02_hashmap_macro` | Build a `hashmap!` macro |
| `ex03_colored_print` | Colored terminal output via ANSI macros |
| `ex04_test_suite` (BONUS) | Small test harness macro |

## Project: `macro_madness`

A CLI tool that demonstrates and exercises custom macros, with `--dump` to show expansion.
