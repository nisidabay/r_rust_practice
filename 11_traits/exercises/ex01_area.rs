// ex01_area.rs — Trait Area { fn area(&self) -> f64 }
//
// Implement for Rectangle, Circle, Triangle.
// Sum areas in Vec<Box<dyn Area>>
//
// Usage: ./ex01_area
// Expected output: areas of each shape, then total

use std::f64::consts;

trait Area {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        consts::PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

fn print_area(shape: &dyn Area) {
    println!("  Area: {:.2}", shape.area());
}

fn main() {
    let shapes: Vec<Box<dyn Area>> = vec![
        Box::new(Rectangle { width: 5.0, height: 3.0 }),
        Box::new(Circle { radius: 4.0 }),
        Box::new(Triangle { base: 6.0, height: 4.0 }),
        Box::new(Rectangle { width: 2.5, height: 4.0 }),
        Box::new(Circle { radius: 1.0 }),
    ];

    println!("Shape areas:");
    let mut total = 0.0;
    for shape in &shapes {
        print_area(shape.as_ref());
        total += shape.area();
    }

    println!("\nTotal area: {:.2}", total);
}
