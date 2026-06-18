// 06_derive_traits.rs
// #[derive(Debug, Clone, Copy, PartialEq, Eq)] — auto-implementing common traits.
//
// Derive macros automatically implement traits for structs and enums when all
// fields already implement that trait. This saves enormous boilerplate.
//
// Run: rustc --edition 2021 06_derive_traits.rs && ./06_derive_traits

use std::fmt;

// ── Basic derived traits ─────────────────────────────────────────────────────

/// Debug  -> {:?} formatting for debugging
/// Clone  -> .clone() creates a deep copy
/// Copy   -> implicit bitwise copy on assignment (opt-in, must be Clone)
/// PartialEq -> == and != comparisons
/// Eq     -> marker trait for total equality (no extra methods; enables HashSet etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector2 {
    x: i32,
    y: i32,
}

// ── Ord and PartialOrd are separate ──────────────────────────────────────────

/// Adding PartialOrd (so we can compare with <, >, <=, >=) and Ord (total order).
/// Ord requires Eq and PartialOrd. Comparison is lexicographic (first field,
/// then second field, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    row: u32,
    col: u32,
}

// ── Default trait for default values ─────────────────────────────────────────

#[derive(Debug, Clone, Copy, Default)]
struct Settings {
    volume: u8,
    brightness: u8,
    theme_id: u32,
}

// ── Hash trait (needed for HashMap/HashSet keys) ─────────────────────────────

use std::collections::HashSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct UserId(u64);

// ── What if a field doesn't support a trait? ──────────────────────────────────

/// We can derive Debug even if a field is a custom type, as long as that type
/// implements Debug. Here, Vector2 already has Debug, so this works.
#[derive(Debug, Clone, PartialEq)]
struct Line {
    start: Vector2,
    end: Vector2,
}

// ── Manual implementations coexist with derives ──────────────────────────────

/// A struct where we derive most things but manually implement Display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// Manual Display
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

fn main() {
    // Debug, Clone, Copy
    let v1 = Vector2 { x: 10, y: 20 };
    let v2 = v1; // Copy — v1 is still valid.
    println!("v1 = {v1:?}, v2 = {v2:?}");

    let v3 = v1.clone(); // Clone — explicit.
    println!("v3 (cloned) = {v3:?}");

    // PartialEq
    assert!(v1 == v2);
    assert!(v1 == v3);
    println!("v1 == v2 == v3: all equal ✓");

    // Ord / PartialOrd
    let a = Coord { row: 1, col: 5 };
    let b = Coord { row: 2, col: 0 };
    println!("a = {a:?}, b = {b:?}");
    println!("a < b: {} (expect true)", a < b);
    println!("a > b: {} (expect false)", a > b);

    // Default
    let s: Settings = Default::default();
    println!("default settings: {s:?}");

    // Hash (enables HashSet membership)
    let mut set = HashSet::new();
    set.insert(UserId(42));
    set.insert(UserId(99));
    println!("HashSet contains UserId(42): {}", set.contains(&UserId(42)));

    // Nested derived structs
    let line = Line {
        start: Vector2 { x: 0, y: 0 },
        end: Vector2 { x: 100, y: 100 },
    };
    println!("line: {line:?}");

    // Display alongside derives
    let white = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    println!("white = {white}  (Debug: {white:?})");
}
