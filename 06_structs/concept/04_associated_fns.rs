/*
 * 04_associated_fns.rs — Practical Rust
 *
 * Question: How do I write functions that belong to a type but don't need an instance?
 *
 * Associated functions (also called "static methods") don't take self.
 * Called with Type::function() syntax, not instance.function().
 * Common use: constructors (convention: new()).
 */

fn main() {
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        // Associated function — no self
        // Constructor pattern (convention)
        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }

        // Another constructor
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        // Regular method — takes &self
        fn distance(&self, other: &Point) -> f64 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            (dx * dx + dy * dy).sqrt()
        }
    }

    // Call associated functions with ::
    let p1 = Point::new(3.0, 4.0);
    let origin = Point::origin();

    println!("p1: ({}, {})", p1.x, p1.y);
    println!("origin: ({}, {})", origin.x, origin.y);

    // Call methods on instances
    let dist = p1.distance(&origin);
    println!("Distance from origin: {:.2}", dist);

    // Another example: constructor with validation
    struct Temperature {
        celsius: f64,
    }

    impl Temperature {
        fn new(celsius: f64) -> Temperature {
            Temperature { celsius }
        }

        fn freezing() -> Temperature {
            Temperature { celsius: 0.0 }
        }

        fn boiling() -> Temperature {
            Temperature { celsius: 100.0 }
        }

        fn to_fahrenheit(&self) -> f64 {
            self.celsius * 9.0 / 5.0 + 32.0
        }
    }

    let t = Temperature::new(25.0);
    println!("{}°C = {}°F", t.celsius, t.to_fahrenheit());
    println!("Freezing: {}°F", Temperature::freezing().to_fahrenheit());
    println!("Boiling: {}°F", Temperature::boiling().to_fahrenheit());
}
