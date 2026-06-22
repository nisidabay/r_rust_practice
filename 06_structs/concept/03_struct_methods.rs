/*
 * 03_struct_methods.rs — Practical Rust
 *
 * Question: How do I add functions that belong to my struct?
 *
 * Methods are defined inside an `impl` block.
 * First parameter is always `self`, `&self`, or `&mut self`.
 * Call with dot notation: instance.method().
 * Methods are syntactic sugar for associated functions with self param.
 */

fn main() {
    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Rectangle {
        // &self — borrow immutably (read-only)
        fn area(&self) -> f64 {
            self.width * self.height
        }

        fn perimeter(&self) -> f64 {
            2.0 * (self.width + self.height)
        }

        // &mut self — can modify
        fn scale(&mut self, factor: f64) {
            self.width *= factor;
            self.height *= factor;
        }

        // self — consumes the instance (rare)
        fn destroy(self) {
            println!("Destroying rectangle: {:.1} x {:.1}", self.width, self.height);
            // self is dropped here
        }
    }

    let mut rect = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    println!("Area: {:.1}", rect.area());
    println!("Perimeter: {:.1}", rect.perimeter());

    rect.scale(2.0);
    println!("After scaling: {:.1} x {:.1}", rect.width, rect.height);
    println!("New area: {:.1}", rect.area());

    // Calling a method that takes self (consumes)
    let other = Rectangle { width: 3.0, height: 4.0 };
    other.destroy();
    // println!("{}", other.width); // COMPILE ERROR: consumed
}
