// 10_operator_overload.rs
// Implementing Add, Display, From, and other operator traits.
//
// Rust allows operator overloading via traits in std::ops. You can define
// how +, -, *, /, indexing, etc. work for your custom types.
//
// Run: rustc --edition 2021 10_operator_overload.rs && ./10_operator_overload

use std::fmt;
use std::ops::Add;

// ── Implementing Add ─────────────────────────────────────────────────────────

/// A 2D vector that supports addition.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }
}

/// Overload the + operator. The Add trait has an associated type Output.
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// ── Implementing Add with different types ────────────────────────────────────
//
// You can add a scalar to a Vec2 on the right side.

impl Add<f64> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

// ── Implementing Display ─────────────────────────────────────────────────────

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// ── Implementing From and Into ───────────────────────────────────────────────
//
// From<T> for U defines how to convert T into U. Into<U> is automatically
// provided for any type that implements From<T>.

/// A complex number type.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }
}

/// Convert a real f64 into a Complex (imaginary part = 0).
impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Complex {
            real: value,
            imag: 0.0,
        }
    }
}

/// Convert a (f64, f64) tuple into a Complex.
impl From<(f64, f64)> for Complex {
    fn from((real, imag): (f64, f64)) -> Self {
        Complex { real, imag }
    }
}

// ── Implementing Add on Complex ──────────────────────────────────────────────

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

// ── Implementing Display for Complex ─────────────────────────────────────────

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

// ── Implementing Neg (unary minus) ──────────────────────────────────────────

use std::ops::Neg;

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

// ── Implementing Index and IndexMut ──────────────────────────────────────────

use std::ops::Index;
use std::ops::IndexMut;

impl Index<usize> for Vec2 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vec2 index out of bounds: {i}"),
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vec2 index out of bounds: {i}"),
        }
    }
}

fn main() {
    // Vec2 operators
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    let v3 = v1 + v2; // Add<Vec2>
    println!("{v1} + {v2} = {v3}");

    let v4 = v3 + 10.0; // Add<f64>
    println!("{v3} + 10 = {v4}");

    // Index
    println!("v4[0] = {}, v4[1] = {}", v4[0], v4[1]);

    let mut v5 = Vec2::new(0.0, 0.0);
    v5[0] = 7.0;
    v5[1] = 8.0;
    println!("v5 after index_mut: {v5}");

    // Complex operators
    let c1 = Complex::new(3.0, 4.0);
    let c2 = Complex::new(1.0, -2.0);
    println!("{c1} + {c2} = {}", c1 + c2);

    // From / Into
    let c_from_f64: Complex = 3.14.into(); // Into<Complex>
    println!("From f64: {c_from_f64}");

    let c_from_tuple: Complex = (1.0, 2.0).into();
    println!("From tuple: {c_from_tuple}");

    // Neg (unary minus)
    let cn = -Complex::new(5.0, -3.0);
    println!("- (5 - 3i) = {cn}");
}
