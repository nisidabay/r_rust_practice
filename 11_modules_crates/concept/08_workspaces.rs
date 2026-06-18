//! # Cargo Workspaces
//!
//! This is a **document-only concept file** explaining Cargo workspaces.
//! To create a real workspace, run the commands shown below.
//!
//! ## What are workspaces?
//!
//! A workspace is a set of crates that share a single `Cargo.lock` and
//! output directory. Useful for multi-crate projects (e.g. a library +
//! several tools that depend on it).
//!
//! ## Workspace structure
//!
//! ```text
//! my_workspace/
//! ├── Cargo.toml        # workspace-level manifest
//! ├── Cargo.lock
//! ├── lib_core/
//! │   └── Cargo.toml    # library crate
//! ├── cli_tool/
//! │   └── Cargo.toml    # binary crate
//! └── wasm_frontend/
//!     └── Cargo.toml    # another binary crate
//! ```
//!
//! ## Workspace Cargo.toml
//!
//! ```toml
//! [workspace]
//! members = [
//!     "lib_core",
//!     "cli_tool",
//!     "wasm_frontend",
//! ]
//! resolver = "2"
//! ```
//!
//! ## Commands
//!
//! - `cargo build` — builds all workspace members
//! - `cargo test` — tests all workspace members
//! - `cargo run -p <name>` — run specific member
//! - `cargo add <crate> -p <name>` — add dependency to specific member
//!
//! ## Key points
//!
//! - All members share one `Cargo.lock` (consistent dependency versions)
//! - Members can depend on each other via `path = "../lib_core"`
//! - `cargo test` runs tests for all members
//! - Workspace members must not have a `[workspace]` section of their own
//! - The `[patch]` and `[profile]` sections can be set at workspace level

fn main() {
    println!("Workspaces concept — read the doc-comment above.");
    println!("Try the following steps to create a workspace:");
    println!("  mkdir my_workspace && cd my_workspace");
    println!("  # Create workspace Cargo.toml with [workspace] members = [...]");
    println!("  cargo new lib_core --lib");
    println!("  cargo new cli_tool");
    println!("  cargo new wasm_frontend");
    println!("  # Add lib_core as dependency to cli_tool");
    println!("  cargo build  # builds all members");
}
