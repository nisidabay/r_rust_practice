fn main() {
    // ref and ref mut in patterns — binding to references inside match arms
    // When you match on a reference, you need & or ref to extract correctly

    // --- The issue: matching on references ---
    let value = 42;
    let ref_val = &value; // ref_val: &i32

    // Match on a reference — the pattern uses & to match the reference itself
    match ref_val {
        // &v means "match a reference to an i32, bind the inner i32 to v"
        &v => println!("Dereferenced in pattern: {}", v),
    }

    // Simpler: use dereference in the arm body
    match ref_val {
        v => println!("v is &i32: {}", *v), // explicit dereference
    }

    // --- ref — borrow inside a match arm ---
    // ref creates a reference to the matched value instead of moving it
    let s = Some(String::from("hello"));

    match &s {
        // Without ref: would move String out of s (leaving s unusable)
        // With ref: we get a reference instead
        Some(ref text) => println!("Borrowed: {}", text),
        None => {}
    }
    // s is still usable here because we borrowed, didn't move
    println!("s is still mine: {:?}", s);

    // --- ref mut — mutable borrow inside a match arm ---
    let mut t = Some(String::from("world"));

    match &mut t {
        Some(ref mut text) => {
            text.push_str("!");
            println!("Modified: {}", text);
        }
        None => {}
    }

    // --- Matching on enums with ref — cleaner ---
    enum Message {
        Text(String),
        Number(i32),
    }

    let msgs = vec![
        Message::Text(String::from("hi")),
        Message::Number(42),
    ];

    for msg in &msgs {
        match msg {
            // Match on &Message — each arm pattern uses ref implicitly
            Message::Text(t) => println!("Text: {}", t),
            Message::Number(n) => println!("Number: {}", n),
        }
        // Because we iterated with &msgs (borrowing), no ref needed
    }
}
