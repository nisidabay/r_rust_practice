// ex03_shape_area.rs
// Exercise: Area trait for Circle, Square, Triangle.
//
// Implement an `Area` trait and apply it to different geometric shapes.
// Then write a generic function that computes total area from a slice.
//
// Task:
//   1. Define the Area trait with an area() -> f64 method.
//   2. Implement it for Circle, Square, Triangle.
//   3. Implement it for a generic Rectangle<T> where T: Into<f64>.
//   4. Write a function `total_area` that accepts &[impl Area].
//   5. Compute the total area of a mixed collection using trait objects.
//
// Run: rustc --edition 2021 ex03_shape_area.rs && ./ex03_shape_area

use std::f64::consts::PI;

// ── The Area trait ───────────────────────────────────────────────────────────

trait Area {
    fn area(&self) -> f64;
}

// ── Shape structs ────────────────────────────────────────────────────────────

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

struct Triangle {
    a: f64, // side lengths
    b: f64,
    c: f64,
}

/// A generic rectangle with width and height of potentially different numeric types.
struct Rectangle<T, U> {
    width: T,
    height: U,
}

// ── Area implementations ─────────────────────────────────────────────────────

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        // Heron's formula
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
}

impl<T, U> Area for Rectangle<T, U>
where
    T: Into<f64> + Copy,
    U: Into<f64> + Copy,
{
    fn area(&self) -> f64 {
        let w: f64 = self.width.into();
        let h: f64 = self.height.into();
        w * h
    }
}

// ── Generic function ─────────────────────────────────────────────────────────

/// Returns the total area of all shapes in a slice. Uses impl Trait syntax
/// to accept any type that implements Area.
fn total_area(shapes: &[impl Area]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// ── Mixed collection via trait objects ───────────────────────────────────────

/// Sums areas from a heterogeneous collection of trait objects.
fn total_area_dyn(shapes: &[&dyn Area]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// ── Display helper ───────────────────────────────────────────────────────────

fn print_area<T: Area>(label: &str, shape: &T) {
    println!("  {label}: {:.2}", shape.area());
}

fn main() {
    // Individual shapes
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 4.0 };
    let triangle = Triangle {
        a: 3.0,
        b: 4.0,
        c: 5.0,
    };
    let rectangle = Rectangle {
        width: 3.0f64,
        height: 7.0,
    };
    let rectangle_int = Rectangle {
        width: 6u32,
        height: 4u32,
    };

    println!("Individual areas:");
    print_area("Circle(r=5)", &circle);
    print_area("Square(s=4)", &square);
    print_area("Triangle(3,4,5)", &triangle);
    print_area("Rect(3×7)", &rectangle);
    print_area("Rect(6×4 int)", &rectangle_int);

    // Total area of homogeneous slices
    let circles = vec![Circle { radius: 1.0 }, Circle { radius: 2.0 }, Circle { radius: 3.0 }];
    println!("\nTotal area of 3 circles: {:.2}", total_area(&circles));

    let rects = vec![
        Rectangle {
            width: 2.0,
            height: 3.0,
        },
        Rectangle {
            width: 4.0,
            height: 5.0,
        },
    ];
    println!(
        "Total area of 2 rectangles: {:.2}",
        total_area(&rects)
    );

    // Heterogeneous via trait objects
    let shapes: Vec<&dyn Area> = vec![&circle, &square, &triangle, &rectangle];
    println!(
        "\nTotal area (mixed via dyn Area): {:.2}",
        total_area_dyn(&shapes)
    );

    // ── Assertions ───────────────────────────────────────────────────────────
    assert!((Circle { radius: 1.0 }.area() - PI).abs() < 1e-10);
    assert!((Square { side: 3.0 }.area() - 9.0).abs() < 1e-10);
    assert!((Triangle { a: 3.0, b: 4.0, c: 5.0 }.area() - 6.0).abs() < 1e-10);
    assert!((Rectangle { width: 2.0f64, height: 3.0 }.area() - 6.0).abs() < 1e-10);
    assert!((Rectangle {
        width: 5u32,
        height: 4u32,
    }
    .area() - 20.0)
        .abs()
        < 1e-10);

    println!("\nAll assertions passed! ✓");
}
