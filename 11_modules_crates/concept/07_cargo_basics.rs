//! # Cargo Basics
//!
//! This is a **document-only concept file** explaining Cargo.toml fundamentals.
//! To create a real cargo project, run: `cargo new demo`
//!
//! ## Cargo.toml structure
//!
//! ```toml
//! [package]
//! name = "my_project"
//! version = "0.1.0"
//! edition = "2021"
//! description = "A sample Rust project"
//!
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! clap = "4"
//! ```
//!
//! ## Key sections
//!
//! - `[package]` — metadata: name, version, edition, authors, description
//! - `[dependencies]` — crates from crates.io or git/path sources
//! - `[dev-dependencies]` — only for tests/examples/benchmarks
//! - `[build-dependencies]` — for build.rs scripts
//! - `[features]` — conditional compilation features
//! - `[lib]` — library target configuration
//! - `[[bin]]` — binary target configuration (multiple binaries)
//!
//! ## Useful cargo commands
//!
//! | Command | Description |
//! |---------|-------------|
//! | `cargo new <name>` | Create new project |
//! | `cargo build` | Build (debug) |
//! | `cargo build --release` | Build (optimized) |
//! | `cargo run` | Build and run |
//! | `cargo check` | Check for errors without producing binary |
//! | `cargo test` | Run tests |
//! | `cargo doc --open` | Build and open docs |
//! | `cargo add <crate>` | Add dependency |
//! | `cargo remove <crate>` | Remove dependency |
//!
//! ## Binary vs Library
//!
//! A crate can be:
//! - **Binary** (`src/main.rs`) — executable
//! - **Library** (`src/lib.rs`) — shared code, reusable
//! - **Both** — library with a binary that uses it

fn main() {
    // This file is a runnable placeholder showing cargo concepts.
    // The real action happens with `cargo new` at the command line.
    println!("Cargo basics — read the doc-comment above.");
    println!("Try: cargo new demo && cd demo && cargo build");
}
