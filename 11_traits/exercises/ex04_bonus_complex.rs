// ex04_bonus_complex.rs — Implement Add and Display for Complex
//
// Operator overloading via std::ops traits.
//
// Usage: ./ex04_bonus_complex
// Expected: complex arithmetic with +, Display, Debug

use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }
}

// Implement Display for nice formatting
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag >= 0.0 {
            write!(f, "{} + {}i", self.real, self.imag)
        } else {
            write!(f, "{} - {}i", self.real, -self.imag)
        }
    }
}

// Implement Add — allows `c1 + c2` syntax
impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

// Bonus: Add with reference
impl Add<&Complex> for Complex {
    type Output = Complex;

    fn add(self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

fn main() {
    let c1 = Complex::new(3.0, 4.0);
    let c2 = Complex::new(1.0, -2.0);
    let c3 = Complex::new(-2.0, 5.0);

    println!("Complex numbers:");
    println!("  c1 = {}", c1);
    println!("  c2 = {}", c2);
    println!("  c3 = {}", c3);

    // Using the + operator
    let sum = c1.clone() + c2.clone();
    println!("\nAddition:");
    println!("  c1 + c2 = {}", sum);

    // Using reference Add
    let sum_ref = &c1 + &c2;
    println!("  &c1 + &c2 = {}", sum_ref);

    // Chaining
    let chain = c1.clone() + c2.clone() + c3.clone();
    println!("  c1 + c2 + c3 = {}", chain);

    // Debug implementation
    println!("\nDebug: {:?}", c1);
    println!("Clone + PartialEq:");
    let c4 = c1.clone();
    println!("  c1 == c4? {}", c1 == c4);
    println!("  c1 == c2? {}", c1 == c2);

    println!("\n--- Traits used ---");
    println!("Add:     operator +");
    println!("Display: println! with {{}}");
    println!("Debug:   println! with {{:?}}");
    println!("Clone:   .clone()");
    println!("PartialEq: ==");
}
