fn main() {
    // Vec<T> — a growable array. Like C's dynamic array but safe and built-in.
    // Use Vec when you need an ordered, indexable, resizable collection.

    // --- Create ---
    let mut v: Vec<i32> = Vec::new();     // empty
    let mut w = vec![10, 20, 30, 40, 50]; // macro — most common way

    // --- Push and Pop ---
    v.push(1);
    v.push(2);
    v.push(3);
    println!("After push: {:?}", v);      // [1, 2, 3]

    let last = v.pop();                    // Option<T> — None if empty
    println!("Popped: {:?}", last);        // Some(3)

    // --- Insert and Remove ---
    w.insert(2, 99);                       // insert 99 at index 2
    println!("After insert: {:?}", w);     // [10, 20, 99, 30, 40, 50]

    let removed = w.remove(0);             // remove element at index 0
    println!("Removed: {}, remaining: {:?}", removed, w); // 10, [20, 99, 30, 40, 50]

    // --- Indexing (panics on out of bounds!) ---
    // Direct [] indexing panics if the index is wrong — like C array access
    println!("w[0] = {}", w[0]);           // 20

    // --- Safe access with .get() ---
    // .get() returns Option<&T> — never panics
    match w.get(0) {
        Some(val) => println!("w[0] = {}", val),
        None      => println!("w[0] is out of bounds"), // never reached here
    }
    match w.get(100) {
        Some(val) => println!("w[100] = {}", val),
        None      => println!("w[100] is out of bounds"), // this runs
    }

    // --- Resize ---
    let mut nums = vec![1, 2, 3];
    nums.resize(5, 0);                     // fill new slots with 0
    println!("Resized: {:?}", nums);       // [1, 2, 3, 0, 0]

    // --- Iterate ---
    for item in &w {
        print!("{} ", item);
    }
    println!();

    // --- Length and capacity ---
    println!("len={}, cap={}", w.len(), w.capacity());
}
