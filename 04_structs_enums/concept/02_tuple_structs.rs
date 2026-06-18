// Tuple structs are named tuples — they have a name but no field names.
// Perfect for newtype wrappers and simple groupings.

fn main() {
    // --- Basic tuple struct ---
    // Fields are accessed by position, like regular tuples.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Access by position index.
    println!("Black: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    // Each tuple struct is its OWN type — Color and Point are not interchangeable,
    // even though they have the same field types!
    // Uncommenting the next line would fail to compile:
    // let c: Color = origin;  // ERROR: expected `Color`, found `Point`

    // --- Destructuring tuple structs ---
    let Color(r, g, b) = black;
    println!("Destructured: r={r}, g={g}, b={b}");

    // --- Newtype pattern ---
    // Wrapping a single type creates a distinct type with zero runtime overhead.
    struct Inches(i32);

    fn print_inches(length: Inches) {
        let Inches(value) = length;
        println!("Length: {value} inches");
    }

    fn print_centimeters(length: Inches) {
        println!("Length: {} cm", length.0 * 2);
    }

    let height = Inches(10);
    print_inches(height);
    // height is MOVED into print_inches — can't use it here anymore.
    // But we can create another one.
    let width = Inches(5);
    print_centimeters(width);

    // --- Tuple struct with #[derive(Debug)] ---
    #[derive(Debug)]
    struct Meters(f64);

    let distance = Meters(100.0);
    println!("Distance: {:?}", distance);
    println!("Raw value: {}", distance.0);
}
