#!/usr/bin/env rustc
// HOW DO YOU GROW YOUR PROGRAM WITHOUT DROWNING IN ONE FILE?
//
// One Question: How do you grow your program without drowning in one file?
// Quick Start: rustc --edition 2021 04_modules_subdir.rs && ./04_modules_subdir
// Learning Path: multi-file → modules → cargo → workspaces
// Common Pattern: `mod utils;` loads `utils/mod.rs` for subdirectory modules
// Now Build Your Own: Organize a module tree with subdirectories

/// Demonstrates modules from subdirectories.
/// `mod utils;` will look for `utils.rs` first, then `utils/mod.rs`.
/// We put `mod.rs` in the `utils/` subdirectory.
/// Inside that mod.rs, sub-modules like `utils::strings` and `utils::files`
/// are available.

mod utils;

fn main() {
    // Use functions from the nested modules via paths
    println!("{}", utils::strings::capitalize("hello"));
    println!("File size: {}", utils::files::format_size(1_048_576));
}
