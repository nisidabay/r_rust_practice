/*
 * 06_arrays.rs — Practical Rust
 *
 * Question: How do I store multiple values?
 *
 * [T; N]: fixed-size array (stack). Size is part of the type.
 * Vec<T>: dynamic array (heap). Growable, std library.
 *
 * Both support: indexing [i], .len(), for loops, .iter()
 */

fn main() {
    // --- Fixed-size array [T; N] ---
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    println!("arr length: {}", arr.len());
    println!("arr[0]={}, arr[3]={}", arr[0], arr[3]);

    // Loop over array
    for val in arr {
        print!("{} ", val);
    }
    println!();

    // Shortcut: [value; count]
    let same = [7; 4];   // [7, 7, 7, 7]
    println!("same: {:?}", same);

    // --- Vec<T> (dynamic / growable) ---
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("vec: {:?} (len={})", v, v.len());

    // Macro shortcut
    let mut nums = vec![1, 2, 3, 4, 5];
    nums.push(6);
    println!("nums: {:?}", nums);

    // Pop from end
    let last = nums.pop();
    println!("popped: {:?}, remaining: {:?}", last, nums);

    // --- Common Vec operations ---
    let mut items = vec!["apple", "banana", "cherry"];

    // Check if empty
    println!("is_empty? {}", items.is_empty());

    // Access with get() — returns Option, no panic on out-of-bounds
    match items.get(1) {
        Some(val) => println!("items[1] = {}", val),
        None => println!("index out of bounds"),
    }

    // Insert at position
    items.insert(1, "blueberry");
    println!("after insert: {:?}", items);

    // Remove by index
    let removed = items.remove(2);
    println!("removed: {}, now: {:?}", removed, items);

    // --- Loop with index (enumerate) ---
    let scores = vec![85, 92, 78, 95];
    for (i, score) in scores.iter().enumerate() {
        println!("scores[{}] = {}", i, score);
    }

    // --- Sum / average pattern ---
    let grades = vec![70, 80, 90, 85];
    let sum: i32 = grades.iter().sum();
    let avg = sum as f64 / grades.len() as f64;
    println!("sum={}, avg={:.1}", sum, avg);
}
