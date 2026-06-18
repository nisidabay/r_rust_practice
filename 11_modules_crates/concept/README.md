# 11 — Modules & Crates

## One Question

**How do you grow your program without drowning in one file?**

## Quick Start

```bash
# Multi-file compilation
rustc --edition 2021 01_multiple_files.rs util.rs && ./01_multiple_files

# Single-file modules
rustc --edition 2021 02_modules_basics.rs && ./02_modules_basics

# Module from separate file
rustc --edition 2021 03_modules_file.rs && ./03_modules_file

# Module from subdirectory
rustc --edition 2021 04_modules_subdir.rs && ./04_modules_subdir

# Visibility and re-exports
rustc --edition 2021 05_pub_use.rs && ./05_pub_use

# Use paths, self, super, crate, as
rustc --edition 2021 06_use_paths.rs && ./06_use_paths

# Cargo basics (document-only)
rustc --edition 2021 07_cargo_basics.rs && ./07_cargo_basics

# Workspaces (document-only)
rustc --edition 2021 08_workspaces.rs && ./08_workspaces
```

## Learning Path

1. **01_multiple_files.rs** — Split code across multiple `.rs` files, compile together
2. **02_modules_basics.rs** — Inline modules with `mod { ... }`
3. **03_modules_file.rs** — `mod my_module;` loads from `my_module.rs`
4. **04_modules_subdir.rs** — `mod utils;` loads from `utils/mod.rs` with sub-modules
5. **05_pub_use.rs** — `pub`, `pub(crate)`, `pub use` re-exports
6. **06_use_paths.rs** — `use` paths, `self`, `super`, `crate`, `as` aliases
7. **07_cargo_basics.rs** — Cargo.toml structure, dependencies, commands
8. **08_workspaces.rs** — Workspaces for multi-crate projects

## Common Patterns

| Pattern | Description |
|---------|-------------|
| `mod helpers;` | External module from `helpers.rs` |
| `mod utils;` | Module from `utils/mod.rs` |
| `pub use inner::Type;` | Re-export to flatten API |
| `use crate::module::fn;` | Absolute path from crate root |
| `use super::fn;` | Relative path to parent |
| `use X as Y;` | Alias to avoid name clashes |

## Now Build Your Own

Take a 200+ line program you've written and split it across modules:
1. Group related functions into separate modules (one file per module)
2. Use `pub` to expose only the public API
3. Use `pub use` to create a clean, flat re-export surface
4. Turn it into a Cargo project with `src/main.rs` and `src/lib.rs`
5. Add a workspace if you have multiple related crates
