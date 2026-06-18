# 16 — Unsafe, Raw Pointers, FFI, Transmute, Extern

**One question:** *What do you do when you need to break the rules?*

Rust's safety guarantees are powerful, but sometimes you need to talk to C
libraries, implement low-level data structures, or perform bit-level
reinterpretation. That's where `unsafe` comes in.

## Files

| # | File | Topic |
|---|------|-------|
| 01 | `01_unsafe_intro.rs` | The `unsafe` keyword — what it means, what it enables |
| 02 | `02_raw_pointers.rs` | `*const T`, `*mut T`, dereferencing, pointer arithmetic |
| 03 | `03_unsafe_fns.rs` | Calling and defining unsafe functions |
| 04 | `04_union_ffi.rs` | `#[repr(C)]` unions, `extern "C"` blocks, FFI basics |
| 05 | `05_mut_static.rs` | Mutable static variables, unsafe access |
| 06 | `06_unsafe_trait.rs` | Implementing unsafe traits (Send, Sync manually) |
| 07 | `07_transmute.rs` | `std::mem::transmute` — when to use, HIGHLY DANGEROUS |
| 08 | `08_abi_calling.rs` | `extern "C"` functions, linking to C libraries |

## The 5 Superpowers of `unsafe`

1. **Dereference a raw pointer** (`*const T` / `*mut T`)
2. **Call an unsafe function or method**
3. **Access or modify a mutable static variable**
4. **Implement an unsafe trait**
5. **Access fields of a `union`**

## Golden Rule

`unsafe` does **not** disable the borrow checker. It does **not** make your
code automatically safe or unsafe. It just gives *you* the responsibility
to uphold the invariants the compiler normally guarantees.

## Project

`hex_dump` — a CLI hex dump tool using unsafe for direct memory access.

## Exercises

- **ex01:** Implement `split_at_mut` using raw pointers
- **ex02:** Convert between CString and `*const i8` safely
- **ex03:** Unsafe extern "C" callback function pattern
- **BONUS:** Build a tiny safe API around unsafe FFI
