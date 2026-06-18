#!/usr/bin/env rustc
// HOW DO YOU GROW YOUR PROGRAM WITHOUT DROWNING IN ONE FILE?
//
// One Question: How do you grow your program without drowning in one file?
// Quick Start: rustc --edition 2021 05_pub_use.rs && ./05_pub_use
// Learning Path: multi-file → modules → cargo → workspaces
// Common Pattern: `pub use` re-exports to create a clean public API
// Now Build Your Own: Design a module tree and flatten the public API with pub use

/// Demonstrates visibility (`pub`, `pub(crate)`) and re-exports with `pub use`.

// --- private implementation module ---
mod internal {
    // Only accessible within this crate
    pub(crate) fn setup() {
        println!("[internal] Setting up...");
    }

    // Fully private
    fn secret() {
        println!("This is a secret helper");
    }

    /// Sub-module with useful types
    pub mod types {
        pub struct User {
            pub name: String,
            pub age: u8,
        }

        impl User {
            pub fn new(name: &str, age: u8) -> Self {
                Self { name: name.to_string(), age }
            }
        }
    }
}

// --- public API module with re-exports ---
pub mod api {
    // Re-export User at the `api` level so users write `api::User` not `api::types::User`
    pub use super::internal::types::User;

    pub fn create_user(name: &str, age: u8) -> User {
        // Reference internal via crate root — pub(crate) makes this accessible
        crate::internal::setup();
        User::new(name, age)
    }
}

fn main() {
    // Access User through the flattened public API
    let u = api::create_user("Charlie", 30);
    println!("User: {} (age {})", u.name, u.age);

    // We could also write `api::User` directly:
    let u2 = api::User::new("Diana", 25);
    println!("User: {} (age {})", u2.name, u2.age);

    // The following would NOT compile (private / pub(crate) from outside):
    // internal::setup();
    // internal::types::User;
}
