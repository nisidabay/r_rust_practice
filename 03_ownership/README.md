# 03_ownership — Who owns the data — and what happens when they're done?

> **The one question:** *Who owns the data — and what happens when they're done?*

Ownership is **the** hard concept in Rust. It's the thing that makes Rust unique,
and it's the thing that gives you memory safety without a garbage collector.

## The Three Rules

1. **Each value has exactly ONE owner.**
2. **Ownership can be transferred** (moved) from one owner to another.
3. **When the owner goes out of scope, the value is dropped** (memory freed).

Everything in this group builds on these three rules.

## Structure

```
03_ownership/
├── concept/           # 7 numbered concept files
│   ├── 01_ownership_intro.rs    # Three rules, stack vs heap
│   ├── 02_move_copy.rs          # Copy types vs move semantics
│   ├── 03_move_fn.rs            # Ownership transfer through functions
│   ├── 04_return_ownership.rs   # Returning ownership, tuple pattern
│   ├── 05_references.rs         # &T immutable references
│   ├── 06_mut_ref.rs            # &mut T mutable references
│   └── 07_slices.rs             # &str, &[T] slices
├── exercises/         # 3 exercises + 1 BONUS with Makefile
│   ├── ex01_word_counter.rs     # Borrow a string, count words
│   ├── ex02_string_reverse.rs   # Reverse a String in-place with &mut
│   ├── ex03_split_first.rs      # Split a slice into first + rest
│   ├── ex04_ownership_debug.rs  # BONUS: fix broken ownership code
│   ├── *_solved.rs              # Solved versions for reference
│   └── Makefile                 # make check
├── project/           # CLI mini-app: json_tokenizer
│   ├── Cargo.toml
│   └── src/main.rs
└── README.md          # This file
```

## Concept Map

| File | Topic | Key Idea |
|------|-------|----------|
| `01_ownership_intro` | Ownership rules | Each value has one owner, scope-based dropping |
| `02_move_copy` | Copy vs Move | i32/f64/bool/char are Copy; String/Vec move |
| `03_move_fn` | Functions & ownership | Passing values moves ownership to parameters |
| `04_return_ownership` | Returning ownership | Tuple pattern for return + ownership |
| `05_references` | &T borrow | Borrow without owning, many readers |
| `06_mut_ref` | &mut T borrow | One writer at a time rule |
| `07_slices` | &str, &[T] | Borrow a piece without owning |

## Quick Start

```bash
# Run a concept file
rustc --edition 2021 concept/01_ownership_intro.rs -o /tmp/ownership && /tmp/ownership

# Run all concept files
for f in concept/*.rs; do
    name=$(basename "$f" .rs)
    rustc --edition 2021 "$f" -o "/tmp/$name"
    echo "=== $name ==="
    "/tmp/$name"
done

# Run exercises with solutions
cd exercises && make check

# Run the project
cd project && echo '{"hello": "world"}' | cargo run

# Get project help
cd project && cargo run -- --help
```

## Key Vocabulary

| Term | Meaning |
|------|---------|
| **Owner** | The variable that holds responsibility for dropping a value |
| **Move** | Transfer of ownership (source invalidated) |
| **Copy** | Automatic bitwise duplication (source stays valid) |
| **Clone** | Explicit deep copy of heap data |
| **Borrow** | Use a reference (&T or &mut T) without owning |
| **Drop** | Free memory when owner goes out of scope |
| **Slice** | A fat pointer (ptr + len) into a contiguous sequence |

## Common Ownership Patterns

```rust
// MOVE — heap type, ownership transfers
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED, can't use s1 anymore

// COPY — stack type, automatically duplicated
let x = 42;
let y = x;    // x is COPIED, both x and y are valid

// BORROW — reference, no ownership transfer
let s = String::from("hello");
let len = calculate_length(&s);  // borrows, doesn't own
println!("{}", s);  // s is still valid

// MUTABLE BORROW — one writer at a time
let mut s = String::from("hello");
change(&mut s);
```

## Next Steps

After mastering ownership, move on to **[04_structs_enums](../04_structs_enums/)** —
how to define your own data types.
