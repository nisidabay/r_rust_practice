# 05 Collections

> **One question:** What do you reach for when you need more than one value?

Rust's standard library provides four fundamental collection types that every
Rust programmer uses daily. Unlike arrays and tuples, these live on the heap,
grow and shrink at runtime, and manage their own memory.

## Concepts

| # | File | What you'll learn |
|---|------|-------------------|
| 01 | `concept/01_vec_basics.rs` | `Vec::new`, `vec!`, `push`, `pop`, indexing, `len`, `get` |
| 02 | `concept/02_vec_iterate.rs` | `for` loops, `iter()`, `iter_mut()`, `into_iter()`, `enumerate()` |
| 03 | `concept/03_string_vs_str.rs` | `String` vs `&str`, `push_str`, `format!`, `to_string()`, `+` |
| 04 | `concept/04_string_methods.rs` | `split`, `trim`, `contains`, `replace`, `chars`, `bytes` |
| 05 | `concept/05_hashmap_basics.rs` | `HashMap::new`, `insert`, `get`, `entry`, `or_insert` |
| 06 | `concept/06_hashset.rs` | `HashSet::new`, `insert`, `contains`, `union`, `intersection` |
| 07 | `concept/07_hashmap_iterate.rs` | Iterating HashMap — `for (k, v)`, `keys()`, `values()` |
| 08 | `concept/08_collection_ownership.rs` | How ownership works with Vec and HashMap — moves in/out |

## Exercises

| # | File | Problem |
|---|------|---------|
| 01 | `exercises/ex01_word_freq.rs` | Read text, count word frequencies with HashMap |
| 02 | `exercises/ex02_merge_dedup.rs` | Merge two sorted Vecs, remove duplicates (two-pointer) |
| 03 | `exercises/ex03_palindrome.rs` | Check if string is a palindrome (chars, rev) |
| 04 🏆 | `exercises/ex04_inverted_index.rs` | Build an inverted index from sentences (BONUS) |

```bash
cd exercises && make check
```

## Project: `grocery_list`

A CLI grocery list manager that stores items as a `Vec<String>`. Persists
to a file on disk.

```bash
cd project
cargo run -- add milk
cargo run -- add "ice cream"
cargo run -- list
cargo run -- remove 2
cargo run -- clear
cargo run -- --help
```

## Key Takeaways

- **`Vec<T>`** is the default growable list — use `vec![]` for convenience.
- **`String`** owns UTF-8 text on the heap; **`&str`** borrows a view of it.
- **`HashMap<K, V>`** maps keys to values with `entry().or_insert()` for the
  "insert if missing" pattern.
- **`HashSet<T>`** is a set of unique values — great for dedup and membership.
- **Collections own their elements.** Pushing moves data in; popping/removing
  moves data out. `get()` returns references, not owned values.
