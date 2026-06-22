/*
 * 05_clone.rs — Practical Rust
 *
 * Question: How do I make a deep copy of something?
 *
 * .clone() performs a deep copy — the original is still valid.
 * Use it when you need both the original and a copy.
 * Don't use it when a reference would do — cloning is expensive.
 * Don't use it on Copy types — they're implicitly copied already.
 */

fn main() {
    // Cloning a String
    let original = String::from("hello world");
    let cloned = original.clone(); // deep copy on the heap
    println!("original: {}", original); // still valid!
    println!("cloned:   {}", cloned);   // separate copy

    // Cloning a Vec
    let numbers = vec![1, 2, 3, 4, 5];
    let backup = numbers.clone();
    println!("numbers: {:?}", numbers);
    println!("backup:  {:?}", backup);

    // When to clone vs not clone:
    // BAD — you only READ the data, use a reference
    fn read_only(s: String) { println!("{}", s); } // BAD: takes ownership
    fn read_only_ref(s: &String) { println!("{}", s); } // GOOD: borrows

    // OK — you need to keep the original AND modify a copy
    fn modify_copy(s: &String) -> String {
        let mut copy = s.clone(); // clone because we need modifiable version
        copy.push_str("!!!");
        copy
    }

    let greeting = String::from("hello");
    let excited = modify_copy(&greeting);
    println!("original: {}, modified: {}", greeting, excited);

    // Cloning is NOT needed for Copy types (implicit)
    let x = 42;
    let y = x; // implicit copy, no .clone() needed
    println!("x = {}, y = {}", x, y);
}
