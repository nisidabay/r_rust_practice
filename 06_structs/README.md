# 06 Structs — How to bundle related data together

> Learn Rust as a tool, not a religion. 6 concepts + 4 exercises + 1 project.

---

## Cheatsheet

```rust
// Define a struct
struct Book {
    title: String,
    author: String,
    pages: u32,
}

// Create an instance
let b = Book { title: String::from("Rust"), author: String::from("Me"), pages: 300 };

// Access fields
println!("{} by {}", b.title, b.author);

// Methods
impl Book {
    fn is_long(&self) -> bool { self.pages > 300 }
}

// Associated function (constructor)
impl Book {
    fn new(title: &str, author: &str) -> Self {
        Book { title: title.to_string(), author: author.to_string(), pages: 0 }
    }
}

// Tuple struct
struct Color(u8, u8, u8);
let red = Color(255, 0, 0);

// Struct update syntax
let b2 = Book { pages: 400, ..b };

// Debug output
#[derive(Debug)]
println!("{:?}", my_struct);
```

---

## Concepts (in order)

```bash
rustc --edition 2021 concept/01_struct_basics.rs -o /tmp/s1 && /tmp/s1
rustc --edition 2021 concept/02_tuple_struct.rs -o /tmp/s2 && /tmp/s2
rustc --edition 2021 concept/03_struct_methods.rs -o /tmp/s3 && /tmp/s3
rustc --edition 2021 concept/04_associated_fns.rs -o /tmp/s4 && /tmp/s4
rustc --edition 2021 concept/05_struct_update.rs -o /tmp/s5 && /tmp/s5
rustc --edition 2021 concept/06_debug_display.rs -o /tmp/s6 && /tmp/s6
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_struct_basics.rs` | Define structs, create instances, access fields | Like a lightweight class |
| 02 | `02_tuple_struct.rs` | Named tuples, newtype pattern | Wrapper types without field names |
| 03 | `03_struct_methods.rs` | impl block, &self, &mut self | Methods are functions with self |
| 04 | `04_associated_fns.rs` | Associated functions, ::new() | Constructor pattern: Book::new() |
| 05 | `05_struct_update.rs` | Spread syntax, destructuring | `..other` copies remaining fields |
| 06 | `06_debug_display.rs` | #[derive(Debug)], Display trait | See what's inside your struct |

---

## Exercises

```bash
cd exercises
make run
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_rectangle.rs` | Rectangle area, perimeter, can_hold | Methods, field access |
| 02 | `ex02_book.rs` | Book with is_long() method | Custom method, booleans |
| 03 | `ex03_student.rs` | Student with grades, average, highest | Vec field, iterator methods |
| 🏆 | `ex04_bonus_bank.rs` | BankAccount with withdraw Result | Result return from method |

---

## Project — Gradebook

REPL managing students and grades using structs.

```bash
cd project
cargo run --
> add Alice
> grade Alice 85
> summary
> best
> quit
```

Built with: struct Student, impl methods, Vec<Student>, iterator for best student.
