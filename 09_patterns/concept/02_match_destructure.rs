fn main() {
    // Destructuring — pulling values out of compound types in match arms
    // Works for tuples, structs, and enums

    // --- Destructuring tuples ---
    let point = (10, 20, "origin");
    match point {
        // Destructure by position — bind each element to a name
        (x, y, label) => println!("{}: ({}, {})", label, x, y),
    }

    // Destructure with wildcards — ignore some positions
    let pair = (100, 200);
    match pair {
        (x, _) => println!("First: {}", x), // ignore second
    }

    // --- Destructuring structs ---
    struct Person {
        name: String,
        age: u8,
        city: String,
    }

    let alice = Person {
        name: String::from("Alice"),
        age: 30,
        city: String::from("NYC"),
    };

    match alice {
        // Destructure by field name — order doesn't matter
        Person { name, age, city } => {
            println!("{} is {} from {}", name, age, city);
        }
    }

    // Destructure with .. to ignore remaining fields
    let bob = Person {
        name: String::from("Bob"),
        age: 25,
        city: String::from("LA"),
    };
    match bob {
        Person { name, .. } => println!("Only need name: {}", name),
    }

    // --- Destructuring enums ---
    enum Shape {
        Circle(f64),
        Rect { w: f64, h: f64 },
    }

    let shapes = vec![Shape::Circle(5.0), Shape::Rect { w: 3.0, h: 4.0 }];
    for shape in &shapes {
        match shape {
            Shape::Circle(radius) => println!("Circle r={}", radius),
            Shape::Rect { w, h } => println!("Rect {}x{}", w, h),
        }
    }
}
