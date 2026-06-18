// 05_static_lifetime.rs
// The 'static lifetime — data that lives for the ENTIRE program.
//
// A 'static reference is valid for the entire duration of the program.
// Think of it as "this reference will never become invalid while the program runs."
//
// Two common cases:
//   1. String literals (&'static str) — embedded in the binary
//   2. const / static items — stored in the data section
//
// IMPORTANT: 'static does NOT mean "immortal" for generic bounds.
// When used as a trait bound (T: 'static), it just means "T has no
// non-'static borrowed data" — it can be owned or contain only static refs.
//
// Run: rustc --edition 2021 05_static_lifetime.rs && ./05_static_lifetime

use std::fmt::Display;

/// A constant string — this lives in the binary's data section.
/// It has type &'static str (the 'static is implicit).
const PROGRAM_NAME: &str = "Lifetime Explorer";

/// A static variable — also 'static, but mutable statics need unsafe to write.
static GREETING: &str = "Welcome to lifetime exploration!";

/// String literals in function returns are &'static str.
fn get_version() -> &'static str {
    "0.1.0"
}

/// A function that explicitly requires 'static input.
fn print_static(s: &'static str) {
    println!("[Static] {s}");
}

/// When 'static is used as a generic bound: T must not borrow non-static data.
/// Practically, this means T is owned (String, i32, Vec<u8>) or has only
/// 'static references inside it.
fn store_for_processing<T: 'static>(value: T) -> Stored<T> {
    Stored { value }
}

#[derive(Debug)]
struct Stored<T: 'static> {
    value: T,
}

/// A struct that holds a &'static string.
#[derive(Debug)]
struct Config {
    name: &'static str,
    version: &'static str,
}

/// Associated CONSTANT — always 'static.
impl Config {
    const DEFAULT: Config = Config {
        name: "default",
        version: "0.0.0",
    };
}

/// Demonstrate the difference between heap-allocated and static strings.
fn analyze_ref(s: &str) {
    // We can check if a reference is 'static at runtime.
    // This is a simple heuristic — we can't truly know at runtime,
    // but we can show the pointer address.
    println!("  reference points to: {:p}", s);
    println!("  length: {}, content: '{s}'", s.len());
}

fn main() {
    // --- &'static str from literals ---
    let static_str: &'static str = "I live in the binary";
    println!("String literal: '{static_str}' (type: &'static str)");

    // --- Constant ---
    println!("Program name: \"{PROGRAM_NAME}\" (type: &'static str)");

    // --- Static variable ---
    println!("Greeting: \"{GREETING}\" (type: &'static str)");

    // --- Function returning &'static str ---
    let version: &'static str = get_version();
    println!("Version: {version}");

    // --- &'static str vs owned String ---
    let static_s: &'static str = "I live forever";
    let owned_s: String = String::from("I live on the heap");

    println!("\nStatic string:");
    analyze_ref(static_s);

    println!("Owned string (reference):");
    analyze_ref(&owned_s);

    // --- Static string can be passed anywhere ---
    print_static(static_s); // OK — it IS 'static
    // print_static(&owned_s); // ERROR: &String is not &'static str

    // --- T: 'static bound in generics ---
    let stored_i32: Stored<i32> = store_for_processing(42);
    println!("\nStored i32: {:?}", stored_i32);

    let stored_string: Stored<String> = store_for_processing("hello".to_string());
    println!("Stored String: {:?}", stored_string);

    // A Vec is also 'static-compatible (it owns its data)
    let stored_vec: Stored<Vec<i32>> = store_for_processing(vec![1, 2, 3]);
    println!("Stored Vec: {:?}", stored_vec);

    // --- Config with 'static fields ---
    let cfg = Config {
        name: "My Program",
        version: "1.2.3",
    };
    println!("\nConfig: {} v{}", cfg.name, cfg.version);

    // --- Default config uses associated constant ---
    let default_cfg: Config = Config::DEFAULT;
    println!("Default Config: {} v{}", default_cfg.name, default_cfg.version);

    // --- Trait objects with 'static ---
    // A Box<dyn Display + 'static> means "the trait object owns all its data"
    let displayable: Box<dyn Display + 'static> = Box::new(String::from("I'm in a trait object"));
    println!("\nTrait object: {displayable}");

    // --- Practical example: config loader ---
    // Load configuration from string literals (common for default configs)
    let config_str: &'static str = "host=localhost\nport=8080";
    let lines: Vec<&str> = config_str.lines().collect();
    println!("\nConfig from static string:");
    for line in lines {
        println!("  {line}");
    }

    println!("\n✓ 'static lifetime understood!");
}
