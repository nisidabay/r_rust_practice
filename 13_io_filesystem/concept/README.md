# 13: io_filesystem — How does Rust talk to the outside world?

**One Question:** How does Rust talk to the outside world?

## Quick Start

```bash
# Compile and run each concept file
rustc --edition 2021 concept/01_stdin_stdout.rs -o concept/01_stdin_stdout && ./concept/01_stdin_stdout
rustc --edition 2021 concept/02_read_file.rs  -o concept/02_read_file  && echo "" | ./concept/02_read_file
rustc --edition 2021 concept/03_write_file.rs -o concept/03_write_file && ./concept/03_write_file
rustc --edition 2021 concept/04_buffered_io.rs -o concept/04_buffered_io && ./concept/04_buffered_io
rustc --edition 2021 concept/05_path_buf.rs   -o concept/05_path_buf   && ./concept/05_path_buf
rustc --edition 2021 concept/06_dir_entries.rs -o concept/06_dir_entries && ./concept/06_dir_entries
rustc --edition 2021 concept/07_env_args.rs   -o concept/07_env_args   && ./concept/07_env_args
rustc --edition 2021 concept/08_metadata.rs   -o concept/08_metadata   && ./concept/08_metadata
```

## Learning Path

1. **01_stdin_stdout.rs** — `println!`, `print!`, `eprintln!`, `read_line`, flushing
2. **02_read_file.rs** — `fs::read_to_string`, `File::open`, `read_to_end`, chunk reads
3. **03_write_file.rs** — `fs::write`, `File::create`, `write!`, `writeln!`, `OpenOptions`
4. **04_buffered_io.rs** — `BufReader`, `BufWriter`, `BufRead` traits, `lines()`, `read_until`, `split`
5. **05_path_buf.rs** — `Path`, `PathBuf`, `join`, `parent`, `file_name`, `extension`, `exists`
6. **06_dir_entries.rs** — `fs::read_dir`, recursive tree walking, filtering by extension
7. **07_env_args.rs** — `std::env::args`, `std::env::var`, `current_dir`, compile-time constants
8. **08_metadata.rs** — `fs::metadata`, file type, size, permissions, timestamps, `PermissionsExt`

## Common Patterns

| Pattern | Code |
|---|---|
| Read file to string | `fs::read_to_string(path)?` |
| Write string to file | `fs::write(path, content)?` |
| Read stdin line | `io::stdin().read_line(&mut s)?` |
| Buffered line reading | `BufReader::new(file).lines()` |
| Append to file | `OpenOptions::new().append(true).open(path)?` |
| Join paths | `base.join("sub").join("file.txt")` |
| Walk directory tree | Recursive `fs::read_dir()` |
| Get env variable | `env::var("HOME")` |
| Parse CLI arg | `env::args().nth(1)` |
| File metadata | `fs::metadata(path)?.len()` |

## Now Build Your Own

- Build a `head` clone that reads a file and prints the first N lines
- Build a simple config reader that parses `key = value` pairs
- Build a recursive file tree printer with sizes
- Build a `diff`-like tool comparing two files line by line
- Build a `tail -f` clone that watches a file for new content
