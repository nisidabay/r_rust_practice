// 02_move_closure.rs — move closures with threads, ownership transfer
//
// The one question: "How do you run code in parallel — safely?"
// Answer: Use move closures to transfer ownership of data into threads.
//
// Rust's ownership rules apply to threads. If a closure captures a variable
// by reference, the compiler can't guarantee the reference outlives the thread.
// Solution: use `move` to transfer ownership of captured values into the closure.

use std::thread;

fn main() {
    // --- Problem: closure capturing a reference ---
    let data = vec![1, 2, 3, 4, 5];

    // This won't compile — the closure borrows `data` by reference, but the
    // compiler can't guarantee `data` lives long enough for the thread.
    // let handle = thread::spawn(|| {
    //     println!("data: {data:?}");
    // });

    // --- Solution: move closure ---
    // The `move` keyword forces the closure to take ownership of `data`.
    let handle = thread::spawn(move || {
        println!("data (moved!): {data:?}");
        // data is now owned by this closure / this thread
    });
    handle.join().unwrap();

    // After the move, `data` is no longer accessible in the main thread:
    // println!("{data:?}"); // ERROR: borrow of moved value

    // --- Moving multiple values ---
    let name = String::from("worker");
    let count = 100u64;

    let handle2 = thread::spawn(move || {
        println!("{name} will count to {count}");
    });
    handle2.join().unwrap();

    // Both `name` and `count` were moved (count is Copy, so still usable)

    // --- Moving a value that is Copy ---
    let x: i32 = 42; // i32 is Copy
    let handle3 = thread::spawn(move || {
        println!("x inside thread: {x}");
    });
    handle3.join().unwrap();
    // x is Copy, so it's still usable here:
    println!("x in main thread: {x}");

    // --- Moving part of a collection into each thread ---
    let strings = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
    ];

    let mut handles = vec![];
    for s in strings {
        // Move `s` (a single String) into the thread closure
        handles.push(thread::spawn(move || {
            println!("Processing: {s}");
            s.len() // return length
        }));
    }

    for h in handles {
        let len = h.join().unwrap();
        println!("Got length: {len}");
    }

    // strings is no longer available — it was consumed by the for loop

    // --- Clone-then-move pattern ---
    let shared = vec![10, 20, 30];
    let mut clone_handles = vec![];

    for i in 0..3 {
        // Clone the data before moving the clone into the thread
        let clone = shared.clone();
        clone_handles.push(thread::spawn(move || {
            println!("Thread {i} sees: {clone:?}");
        }));
    }

    for h in clone_handles {
        h.join().unwrap();
    }
    // shared is still alive here because we cloned for each thread
    println!("shared is still here: {shared:?}");

    println!("All move closure examples done!");
}
