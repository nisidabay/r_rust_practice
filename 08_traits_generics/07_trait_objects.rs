// 07_trait_objects.rs
// Box<dyn Trait> and dynamic dispatch.
//
// Trait objects enable runtime polymorphism: you can store values of different
// types that all implement the same trait behind a single pointer. The actual
// method called is determined at runtime (dynamic dispatch via vtable) rather
// than at compile time (monomorphization).
//
// Run: rustc --edition 2021 07_trait_objects.rs && ./07_trait_objects

use std::f64::consts::PI;

// ── Define a trait ──────────────────────────────────────────────────────────

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &str;
}

// ── Implementors ─────────────────────────────────────────────────────────────

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
    fn name(&self) -> &str {
        "Circle"
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
    fn name(&self) -> &str {
        "Square"
    }
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // Heron's formula
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
    fn name(&self) -> &str {
        "Triangle"
    }
}

// ── Functions accepting trait objects ────────────────────────────────────────

/// Takes a trait object by reference &dyn Shape — fat pointer (data ptr + vtable ptr).
fn print_shape_info(shape: &dyn Shape) {
    println!(
        "{} — area: {:.2}, perimeter: {:.2}",
        shape.name(),
        shape.area(),
        shape.perimeter()
    );
}

// ── Vec<Box<dyn Trait>> — heterogeneous collection ──────────────────────────

/// Sums the areas of all shapes in a heterogeneous collection.
fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// ── Object safety ───────────────────────────────────────────────────────────
//
// Only "object-safe" traits can be used as trait objects. A trait is object-safe
// if all its methods:
//   - Do not return Self (except by Box<Self>)
//   - Do not have generic type parameters
//   - Are not static (no Self: Sized requirement)
//
// Object-safe trait:
trait ObjectSafe {
    fn describe(&self) -> String; // OK: takes &self, no generics
}
// Not object-safe:
// trait NotObjectSafe {
//     fn from_raw(raw: &[u8]) -> Self;        // ❌ returns Self
//     fn filter<T: PartialEq>(&self, x: T);   // ❌ generic parameters
// }

fn main() {
    // ── &dyn Trait references ────────────────────────────────────────────────
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 4.0 };
    let triangle = Triangle {
        a: 3.0,
        b: 4.0,
        c: 5.0,
    };

    print_shape_info(&circle as &dyn Shape);
    print_shape_info(&square);
    print_shape_info(&triangle);

    // ── Box<dyn Trait> — owned heap-allocated trait objects ──────────────────
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Square { side: 3.0 }),
        Box::new(Triangle {
            a: 5.0,
            b: 12.0,
            c: 13.0,
        }),
    ];

    println!("\nHeterogeneous shapes collection:");
    for shape in &shapes {
        print_shape_info(shape.as_ref());
    }

    println!("\nTotal area: {:.2}", total_area(&shapes));

    // ── Dynamic dispatch overhead ────────────────────────────────────────────
    // &dyn Shape is a "fat pointer" (2 words: data ptr + vtable ptr).
    // Each method call goes through the vtable — slightly slower than static dispatch.
    println!("\nSize of &dyn Shape: {} bytes", std::mem::size_of::<&dyn Shape>());
    println!("Size of &Circle:   {} bytes", std::mem::size_of::<&Circle>());
}
