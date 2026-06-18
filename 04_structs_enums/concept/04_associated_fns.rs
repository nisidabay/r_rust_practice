// Associated functions are functions in an `impl` block that DON'T take `self`.
// They're called with `Type::function()` syntax — constructors are the classic use.

fn main() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // --- Associated functions ---
    impl Rectangle {
        // Constructor: creates a new Rectangle (by convention called `new`).
        fn new(width: u32, height: u32) -> Self {
            // `Self` is an alias for the type (Rectangle).
            Self { width, height }
        }

        // Another constructor: creates a square.
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }

        // Method (takes &self) — not an associated function.
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    // Call associated functions with `::` syntax.
    let rect = Rectangle::new(10, 20);
    println!("10x20 area: {}", rect.area());

    let square = Rectangle::square(15);
    println!("15x15 area: {}", square.area());

    // --- Multiple constructors pattern ---
    struct Temperature {
        celsius: f64,
    }

    impl Temperature {
        // Named constructors make the API clear.
        fn from_celsius(c: f64) -> Self {
            Self { celsius: c }
        }

        fn from_fahrenheit(f: f64) -> Self {
            Self {
                celsius: (f - 32.0) * 5.0 / 9.0,
            }
        }

        fn to_fahrenheit(&self) -> f64 {
            self.celsius * 9.0 / 5.0 + 32.0
        }
    }

    let boiling = Temperature::from_celsius(100.0);
    println!("100°C = {}°F", boiling.to_fahrenheit());

    let freezing = Temperature::from_fahrenheit(32.0);
    println!("32°F = {}°C", freezing.celsius);

    // --- Using `Self` in return position ---
    // `Self` always refers to the type the impl block is for.
    #[derive(Debug)]
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Self {
            Self { count: 0 }
        }

        fn increment(&mut self) {
            self.count += 1;
        }
    }

    let mut c = Counter::new();
    c.increment();
    c.increment();
    println!("Counter: {}", c.count);
}
