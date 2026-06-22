// 02_capture.rs — Closures capture variables from their environment
//
// Three modes: by reference (&), by mutable reference (&mut), by value (move).
// The compiler picks the LEAST invasive mode that still works.

fn main() {
    // CAPTURE BY REFERENCE (default when closure only reads)
    let name = String::from("Alice");
    let greet = || println!("Hello, {}!", name);
    greet();           // closure borrows name immutably
    greet();           // can call multiple times
    println!("Still have name: {}", name);  // name is still accessible

    // CAPTURE BY MUTABLE REFERENCE (when closure mutates)
    let mut counter = 0;
    let mut increment = || {
        counter += 1;  // requires &mut counter
    };
    increment();
    increment();
    // Can't borrow counter immutably while mutable borrow exists
    // println!("counter = {}", counter);  // ERROR: borrowed as mutable
    println!("counter incremented (can't read while mut ref active)");

    // After closure is used last time, borrow ends
    drop(increment);   // explicit drop ends the mutable borrow
    println!("counter = {}", counter);  // now accessible again

    // CAPTURE BY VALUE (when type is Copy or closure consumes)
    let text = String::from("I will be moved");
    let consume = || {
        let _ = text;  // text moved into closure by value
    };
    consume();
    // println!("text = {}", text);  // ERROR: value used after move

    // Showing all three modes in a single example
    let read_only = String::from("read");
    let mut writable = String::from("write");
    let owned = String::from("own");

    let mixed = || {
        println!("read: {}", read_only);   // by ref
        writable.push_str("!");            // by mut ref
        drop(owned);                       // by value
    };
    mixed();
    // owned is gone, writable is mutated, read_only is still readable
    println!("Writable now: '{}'", writable);

    println!("\nCapture modes: compiler picks the least invasive one that works.");
}
