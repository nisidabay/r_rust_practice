/*
 * 02_tuple_struct.rs — Practical Rust
 *
 * Question: What if I just want a new type wrapper without named fields?
 *
 * Tuple struct: struct Name(Type1, Type2, ...)
 * Fields accessed by index: .0, .1, .2
 * Newtype pattern: struct Wrapper(Type) — wraps a single type
 * Useful for type safety (distinguish different kinds of IDs, etc.)
 */

fn main() {
    // Tuple struct with two fields
    struct Color(u8, u8, u8); // RGB

    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);

    // Access by index
    println!("Red: ({}, {}, {})", red.0, red.1, red.2);
    println!("Green: ({}, {}, {})", green.0, green.1, green.2);

    // Newtype pattern — wrap a single type for type safety
    struct UserId(u64);
    struct OrderId(u64);

    let user = UserId(42);
    let order = OrderId(12345);

    // These are DIFFERENT types — can't accidentally mix them
    // let sum = user.0 + order.0; // Would need explicit extraction
    println!("User ID: {}, Order ID: {}", user.0, order.0);

    // Tuple struct with mixed types
    struct Point2D(f64, f64);

    let origin = Point2D(0.0, 0.0);
    let p = Point2D(3.0, 4.0);

    // Destructuring
    let Point2D(x, y) = p;
    println!("Point: ({}, {})", x, y);

    // Distance from origin
    let dist = (x.powi(2) + y.powi(2)).sqrt();
    println!("Distance from origin: {:.2}", dist);
}
