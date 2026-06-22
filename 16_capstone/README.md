# 16 Capstone — Putting it all together

> Everything from groups 01–15 comes together in one final project.

---

## Cheatsheet

```rust
// CLI args
let args: Vec<String> = env::args().collect();

// File persistence
fs::read_to_string("path.txt")?;
fs::write("path.txt", &data)?;

// Error chaining
let data = read_file()?;
let parsed = parse_data(&data)?;
let validated = validate(&parsed)?;

// Struct app pattern
struct App {
    items: Vec<Item>,
    path: String,
}
impl App {
    fn add(&mut self, text: String) { /* ... */ }
    fn save(&self) -> Result<()> { /* ... */ }
}
```

---

## Concepts (in order)

```bash
rustc --edition 2021 concept/01_cli_args.rs -o /tmp/c1 && /tmp/c1
rustc --edition 2021 concept/02_file_io.rs -o /tmp/c2 && /tmp/c2
rustc --edition 2021 concept/03_result_chain.rs -o /tmp/c3 && /tmp/c3
rustc --edition 2021 concept/04_struct_app.rs -o /tmp/c4 && /tmp/c4
rustc --edition 2021 concept/05_everything_together.rs -o /tmp/c5 && /tmp/c5
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_cli_args.rs` | Parse CLI arguments manually | env::args(), pattern match flags |
| 02 | `02_file_io.rs` | Read/write struct state to file | Manual JSON serialization |
| 03 | `03_result_chain.rs` | Chain operations with ? | Read → parse → validate → write |
| 04 | `04_struct_app.rs` | App struct with main loop | State mutation, menu loop |
| 05 | `05_everything.rs` | Complete mini-app | Contacts with file persistence |

---

## Exercises

```bash
cd exercises
make run
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_grep.rs` | Search for pattern in stdin | CLI args, BufRead, --ignore-case |
| 02 | `ex02_cat.rs` | Cat with line numbers, binary detection | File IO, args, byte check |
| 03 | `ex03_tee.rs` | Read stdin, write to file + stdout | Write to multiple destinations |
| 🏆 | `ex04_bonus_cut.rs` | Extract columns by delimiter | Split, filter, join |

---

## Project — Todo List Manager

FINAL PROJECT. Uses ALL concepts from groups 01–15.

```bash
cd project
cargo run -- add "Buy milk" --priority high
cargo run -- list
cargo run -- done 1
cargo run -- export --format csv
```

### Features

- **add**: Add task with optional --priority, --tag, --due
- **list**: Show all tasks with status
- **done <id>**: Mark task complete
- **remove <id>**: Delete task
- **export --format csv|json**: Export to file

### Architecture

- `Todo` struct with fields, `App` struct with methods
- Enum `TodoStatus`, custom `TodoError` enum
- File persistence (manual JSON, no serde)
- HashMap by tag, Display trait for output
- `match` on commands, `?` for error propagation
- Iterator filtering, formatting with alignment
- Full test suite in tests/
