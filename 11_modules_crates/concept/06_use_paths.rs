#!/usr/bin/env rustc
// HOW DO YOU GROW YOUR PROGRAM WITHOUT DROWNING IN ONE FILE?
//
// One Question: How do you grow your program without drowning in one file?
// Quick Start: rustc --edition 2021 06_use_paths.rs && ./06_use_paths
// Learning Path: multi-file → modules → cargo → workspaces
// Common Pattern: `use` with `self`, `super`, `crate`, and `as` for clean paths
// Now Build Your Own: Refactor a deeply nested module tree with use aliases

/// Demonstrates `use` paths, `self`, `super`, `crate`, and `as` aliases.

// A nested module tree to demonstrate path navigation
mod outer {
    pub mod inner {
        pub fn deep_function() -> &'static str {
            "I'm deep!"
        }

        pub struct Config {
            pub name: String,
        }

        impl Config {
            pub fn new(name: &str) -> Self {
                Self { name: name.to_string() }
            }
        }
    }

    pub fn factory() -> &'static str {
        "outer factory"
    }
}

// Another sibling module that uses `super` and `crate` paths
mod sibling {
    use crate::outer::inner::Config;

    pub fn make_config(name: &str) -> Config {
        Config::new(name)
    }

    pub fn describe() -> &'static str {
        // `super` refers to the crate root (same as `crate::` here)
        super::outer::factory()
        // equivalent: crate::outer::factory()
    }
}

fn main() {
    // --- Full path ---
    println!("{}", crate::outer::inner::deep_function());

    // --- `use` to bring into scope ---
    use crate::outer::inner::deep_function;
    println!("{}", deep_function());

    // --- `use ... as` alias ---
    use crate::outer::inner::Config as Cfg;
    let cfg = Cfg::new("myapp");
    println!("Config name: {}", cfg.name);

    // --- Import multiple items ---
    use crate::outer::inner::{deep_function as df, Config};
    let cfg2 = Config::new("other");
    println!("{} for {}", df(), cfg2.name);

    // --- Using sibling module ---
    let cfg3 = sibling::make_config("from sibling");
    println!("Sibling config: {}", cfg3.name);
    println!("Sibling describes: {}", sibling::describe());
}
