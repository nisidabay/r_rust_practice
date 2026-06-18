// ex02_to_any_base.rs
// Exercise: Trait to convert numbers to any base string.
//
// Implement a `ToBase` trait that converts integer types to a string
// representation in any base from 2 to 36.
//
// Task: Define the trait and implement it for at least u32 and i32.
//       Handle negative numbers for signed types.
//
// Run: rustc --edition 2021 ex02_to_any_base.rs && ./ex02_to_any_base

use std::fmt;

// ── The trait ────────────────────────────────────────────────────────────────

/// A trait that converts a numeric value to a string in a given base.
trait ToBase {
    /// Convert self to a string in the given base (2..=36).
    /// Panics if base is not in range 2..=36.
    fn to_base(&self, base: u32) -> String;

    /// Convert to binary (base 2) — convenience method.
    fn to_binary(&self) -> String {
        self.to_base(2)
    }

    /// Convert to hexadecimal (base 16) — convenience method.
    fn to_hex(&self) -> String {
        self.to_base(16)
    }
}

// ── Helper: convert a single digit 0..=35 to a char ──────────────────────────

fn digit_to_char(digit: u32) -> char {
    match digit {
        0..=9 => (b'0' + digit as u8) as char,
        10..=35 => (b'a' + (digit - 10) as u8) as char,
        _ => unreachable!(),
    }
}

// ── Implement for u32 (unsigned) ─────────────────────────────────────────────

impl ToBase for u32 {
    fn to_base(&self, base: u32) -> String {
        assert!((2..=36).contains(&base), "base must be between 2 and 36");
        if *self == 0 {
            return "0".to_string();
        }
        let mut n = *self;
        let mut digits = Vec::new();
        while n > 0 {
            digits.push(digit_to_char(n % base));
            n /= base;
        }
        digits.iter().rev().collect()
    }
}

// ── Implement for i32 (signed — handle negatives) ────────────────────────────

impl ToBase for i32 {
    fn to_base(&self, base: u32) -> String {
        assert!((2..=36).contains(&base), "base must be between 2 and 36");
        if *self == 0 {
            return "0".to_string();
        }
        let negative = *self < 0;
        // Work with absolute value as u32
        let abs = self.unsigned_abs();
        let s = abs.to_base(base);
        if negative {
            format!("-{s}")
        } else {
            s
        }
    }
}

// ── Implement for u8 (smaller unsigned) ──────────────────────────────────────

impl ToBase for u8 {
    fn to_base(&self, base: u32) -> String {
        (*self as u32).to_base(base)
    }
}

// ── Implement for i8 (smaller signed) ────────────────────────────────────────

impl ToBase for i8 {
    fn to_base(&self, base: u32) -> String {
        (*self as i32).to_base(base)
    }
}

// ── Print helper using Display-style ─────────────────────────────────────────

fn print_conversions<T: ToBase + fmt::Display>(label: &str, values: &[T]) {
    println!("{label}:");
    for v in values {
        println!(
            "  {v:>10}  bin={:>12}  hex={:>4}  base5={}",
            v.to_binary(),
            v.to_hex(),
            v.to_base(5)
        );
    }
}

fn main() {
    // Unsigned
    print_conversions("u32", &[0u32, 1, 10, 42, 255, 1024, 65535]);

    // Signed (including negative)
    print_conversions(
        "i32",
        &[0i32, 1, -1, 42, -42, 127, -128, 1000, -1000],
    );

    // Small types
    print_conversions("u8", &[0u8, 1, 128, 255]);
    print_conversions("i8", &[0i8, 1, -1, 127, -128]);

    // ── Assertions ───────────────────────────────────────────────────────────
    assert_eq!(42u32.to_base(2), "101010");
    assert_eq!(42u32.to_base(16), "2a");
    assert_eq!(255u32.to_base(16), "ff");
    assert_eq!(0u32.to_base(10), "0");
    assert_eq!((-42i32).to_base(2), "-101010");
    assert_eq!((-1i32).to_base(16), "-1");
    assert_eq!(16u32.to_base(4), "100");
    assert_eq!(255u8.to_base(16), "ff");
    assert_eq!((-128i8).to_base(2), "-10000000");

    println!("\nAll assertions passed! ✓");
}
