# 14 I/O — Reading and writing files in Rust

> Rust's std::fs module provides everything you need for file I/O. Buffered readers and writers minimize syscalls. Path and PathBuf handle filesystem paths safely.

---

## Cheatsheet

```rust
use std::fs;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

// Read entire file to String
let contents = fs::read_to_string("file.txt")?;

// Write String to file
fs::write("out.txt", "hello")?;

// Read line by line (large files)
let file = File::open("big.txt")?;
let reader = BufReader::new(file);
for line in reader.lines() {
    let line = line?;
}

// Write efficiently (fewer syscalls)
let file = File::create("out.txt")?;
let mut writer = BufWriter::new(file);
write!(writer, "{}", data)?;
writer.flush()?;

// Path operations
Path::new("dir/file.txt").exists();
Path::new("file.txt").extension();    // Some("txt")
Path::new("/a/b/c").parent();         // Some("/a/b")
```

---

## Concepts (in order, compile and run each)

```bash
cd 14_io
rustc --edition 2021 01_read_file.rs -o /tmp/rf && /tmp/rf
rustc --edition 2021 02_write_file.rs -o /tmp/wf && /tmp/wf
rustc --edition 2021 03_buf_reader.rs -o /tmp/br && /tmp/br
rustc --edition 2021 04_buf_writer.rs -o /tmp/bw && /tmp/bw
rustc --edition 2021 05_path.rs -o /tmp/path && /tmp/path
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_read_file.rs` | Reading files | read_to_string, File::open, Result handling |
| 02 | `02_write_file.rs` | Writing files | fs::write, File::create, append, write! |
| 03 | `03_buf_reader.rs` | Buffered reading | BufReader, lines(), large file processing |
| 04 | `04_buf_writer.rs` | Buffered writing | BufWriter, flush, fewer syscalls |
| 05 | `05_path.rs` | Path handling | Path, PathBuf, join, exists, is_file, canonicalize |

---

## Exercises

```bash
cd 14_io/exercises
make run
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_head.rs` | Print first N lines of file | read_to_string, lines(), take() |
| 02 | `ex02_config_reader.rs` | Parse KEY=VALUE config | BufReader, HashMap, error handling |
| 03 | `ex03_file_stats.rs` | lines/words/chars per file | BufReader, args, aggregating stats |
| 🏆 | `ex04_bonus_find.rs` | Recursive file search | read_dir, Path, recursion |

---

## Project — Log Line Counter

Read a log file, count lines matching levels (INFO, WARN, ERROR). Output summary table.

```bash
cd 14_io/project
cargo run -- path/to/logfile
cargo run -- path/to/logfile --level ERROR --top 5
```

When you see the summary table of log levels, think:
**"Every line was counted with BufReader, every stat accumulated in a HashMap."**

---
