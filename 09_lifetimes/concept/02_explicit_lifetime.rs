// 02_explicit_lifetime.rs
// Explicit lifetimes — when you NEED to write them: multiple references, complex returns.
//
// Lifetime elision handles ~90% of cases. The remaining 10% need explicit annotations.
// The classic case: a function that takes TWO references and returns ONE.
// The compiler can't know which input the output refers to, so YOU must tell it.
//
// Lifetime annotations are ALWAYS the same letter for related references:
//   fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
// means "x, y, and the return value all live at least as long as 'a"
//
// Key rule: the output lifetime can NEVER be longer than the shortest input lifetime.
//
// Run: rustc --edition 2021 02_explicit_lifetime.rs && ./02_explicit_lifetime

use std::fmt::Display;

/// The classic example: return the longer of two string slices.
/// 'a means "the returned reference lives as long as the SHORTER of x and y"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

/// Multiple lifetimes: 'a for the data, no lifetime for the separator.
/// The separator is NOT part of the output, so it doesn't need the same lifetime.
fn join_with<'a>(parts: &[&'a str], sep: &str) -> String {
    let mut result = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            result.push_str(sep);
        }
        result.push_str(part);
    }
    result
}

/// Two different lifetimes: 'a for the string, 'b for the prefix.
/// The function checks if the string starts with prefix.
/// Output is 'a (the string), not 'b (the prefix).
/// Written with 'a and 'b explicitly separated.
fn strip_prefix<'a, 'b>(s: &'a str, prefix: &'b str) -> Option<&'a str> {
    if s.starts_with(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

/// Multiple lifetimes with generic bounds.
/// 'a applies to both the data reference AND the return value.
/// T must be Display (for printing) — note lifetimes pair with generics.
fn announce_and_longest<'a, T: Display>(x: &'a str, y: &'a str, announcement: T) -> &'a str {
    println!("Announcement: {announcement}");
    if x.len() >= y.len() { x } else { y }
}

/// Function-level lifetime with a struct.
/// We pass two parts that must live at least as long as 'a.
fn combine<'a>(first: &'a str, second: &'a str) -> Combined<'a> {
    Combined {
        full: format!("{first}{second}"),
        first,
        second,
    }
}

struct Combined<'a> {
    full: String,     // Owned — no lifetime needed
    first: &'a str,   // Borrowed — needs lifetime
    second: &'a str,  // Borrowed — needs lifetime
}

fn main() {
    // --- longest with two references ---
    let s1 = String::from("long string");
    {
        let s2 = String::from("short");
        let result = longest(&s1, &s2);
        println!("longest('{s1}', '{s2}') = '{result}'");
        // result borrows from s2 (the shorter) — valid in this block
    }
    // result is dropped here ✓

    // --- join_with ---
    let words = ["hello", "world", "from", "Rust"];
    let joined = join_with(&words, " -- ");
    println!("join_with: {joined}");

    // --- strip_prefix with two lifetimes ---
    let path = "/usr/local/bin";
    match strip_prefix(path, "/usr") {
        Some(rest) => println!("strip_prefix('{path}', '/usr') = '{rest}'"),
        None => println!("no match"),
    }

    // Different lifetimes: path lives longer than prefix
    let path2 = String::from("/home/user/docs");
    let prefix2 = String::from("/home");
    // prefix2 COULD be dropped first, and strip_prefix still works correctly
    // because the return references path2 ('a), not prefix2 ('b)
    {
        let temp_prefix = String::from("/home");
        let remaining = strip_prefix(&path2, &temp_prefix);
        // temp_prefix dropped here — but remaining borrows from path2, not temp_prefix ✓
        match remaining {
            Some(r) => println!("strip_prefix with temp prefix: '{r}'"),
            None => println!("no match"),
        }
    }

    // --- announce_and_longest ---
    let a = "Rust programming";
    let b = "Go";
    let result = announce_and_longest(a, b, "Comparing languages!");
    println!("announce_and_longest result: '{result}'");

    // --- combine ---
    let greeting = "Hello, ";
    let name = "Alice";
    let combined = combine(greeting, name);
    println!("combined.first = '{}'", combined.first);
    println!("combined.second = '{}'", combined.second);
    println!("combined.full = '{}'", combined.full);
}
