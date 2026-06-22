// 04_move_keyword.rs — The `move` keyword forces ownership transfer
//
// Normally the compiler picks the least invasive capture mode (by ref).
// `move` forces the closure to take ownership of captured variables.
// This is REQUIRED when the closure outlives the current scope (e.g. threads).

// Example: without `move`, a closure captures by reference.
// But if the closure is sent to a thread, the reference might dangle.

use std::thread;

fn main() {
    // WHY MOVE IS NEEDED: threads can outlive the current scope
    let data = String::from("hello from main");

    // Without `move`: compiler says "closure may outlive the current function"
    // let handle = thread::spawn(|| {
    //     println!("{}", data);  // ERROR: data might not live long enough
    // });

    // With `move`: data is MOVED into the closure, thread owns it
    let handle = thread::spawn(move || {
        println!("Thread says: {}", data);
    });

    // data is now owned by the thread closure — can't use it here
    // println!("{}", data);  // ERROR: value used after move

    handle.join().unwrap();

    // Move on non-Copy types: the String is moved into the closure
    let greeting = String::from("Hello");
    let say_it = move || {
        println!("{}", greeting);  // greeting moved here
    };
    say_it();
    // println!("{}", greeting);  // ERROR: greeting is gone

    // Move on Copy types: they get copied into the closure
    let number = 42;
    let show = move || {
        println!("Number: {}", number);  // i32 is Copy, so it's copied
    };
    show();
    println!("Number still available: {}", number);  // OK! i32 is Copy

    // Practical use: building closures that own their data
    let prefix = String::from(">> ");
    let prefixer = move |s: &str| -> String {
        format!("{}{}", prefix, s)  // prefix moved in, stays with closure
    };
    println!("{}", prefixer("first"));
    println!("{}", prefixer("second"));
    // prefix is gone, but prefixer has its own copy

    println!("\n`move`: forces ownership, necessary for threads and spawned tasks.");
}
