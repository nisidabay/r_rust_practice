fn main() {
    // Enum variants can hold data — like a tagged union in C, but type-safe
    enum Message {
        Quit,                       // No data — just a variant
        Move { x: i32, y: i32 },   // Named fields (like a struct)
        Write(String),              // Single unnamed field — tuple variant
        ChangeColor(i32, i32, i32), // Multiple unnamed fields
    }

    let msgs = vec![
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 128),
        Message::Quit,
    ];

    for msg in &msgs {
        // Extract data from each variant using pattern matching
        // The compiler ensures we handle EVERY variant — no forgotten cases
        match msg {
            Message::Quit => println!("  Quit — no data"),
            // Destructure named fields by name
            Message::Move { x, y } => println!("  Move to ({}, {})", x, y),
            // Destructure a single unnamed field
            Message::Write(text) => println!("  Write: \"{}\"", text),
            // _ binds to the rest; we use it to ignore the color values
            Message::ChangeColor(r, g, b) => {
                println!("  ChangeColor to rgb({}, {}, {})", r, g, b);
            }
        }
    }
}
