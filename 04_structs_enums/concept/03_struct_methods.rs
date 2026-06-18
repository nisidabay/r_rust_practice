// Methods are functions attached to a struct via `impl` blocks.
// They take `&self`, `&mut self`, or `self` as the first parameter.

fn main() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // --- impl block with methods ---
    impl Rectangle {
        // Immutable reference — just reads data.
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // Mutable reference — modifies the struct.
        fn scale(&mut self, factor: u32) {
            self.width *= factor;
            self.height *= factor;
        }

        // Takes ownership — destroys the struct.
        fn destroy(self) {
            println!("Destroyed {:?}", self);
            // self is dropped here
        }

        // Multiple parameters beyond self.
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width >= other.width && self.height >= other.height
        }
    }

    let mut rect = Rectangle {
        width: 10,
        height: 20,
    };

    // Call methods using dot notation (automatic referencing).
    println!("Area: {}", rect.area());
    // &rect.area() — Rust auto-refs for method calls!

    println!("Can hold 5x5? {}", rect.can_hold(&Rectangle { width: 5, height: 5 }));

    rect.scale(2);
    println!("After scale: {}x{}", rect.width, rect.height);

    rect.destroy();
    // rect is now moved — can't use it.
    // Uncommenting next line would fail:
    // println!("{:?}", rect);

    // --- Multiple impl blocks ---
    // A type can have multiple `impl` blocks (useful for organization).
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    impl Point {
        fn translate(&mut self, dx: f64, dy: f64) {
            self.x += dx;
            self.y += dy;
        }
    }

    let mut p = Point { x: 3.0, y: 4.0 };
    println!("Distance from origin: {}", p.distance_from_origin());
    p.translate(1.0, 1.0);
    println!("After translate: ({}, {})", p.x, p.y);
}
