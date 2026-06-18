// `for` is Rust's preferred loop construct — it iterates over anything
// that implements `IntoIterator` (ranges, collections, slices, etc.).

fn main() {
    // --- for over a range (exclusive) ---
    // `0..5` is a Range (start inclusive, end exclusive): yields 0,1,2,3,4.
    println!("Range 0..5 (exclusive):");
    for i in 0..5 {
        print!("{i} ");
    }
    println!();

    // --- for over a range (inclusive) ---
    // `1..=5` is a RangeInclusive: yields 1,2,3,4,5.
    println!("Range 1..=5 (inclusive):");
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();

    // --- for over a collection (by reference) ---
    // Iterating over a Vec borrows each element. The Vec remains usable.
    let names = vec!["Alice", "Bob", "Charlie"];
    println!("Iterating over Vec:");
    for name in &names {
        // `name` is `&&str` — a reference to the element.
        print!("{name} ");
    }
    println!();
    // names is still valid here because we only borrowed.
    println!("Vec still usable: {}", names.len());

    // --- for over a collection (by value, consuming) ---
    // Iterating directly (without `&`) consumes the collection.
    let numbers = vec![10, 20, 30];
    for n in numbers {
        // `n` is `i32` — ownership moved into the loop.
        print!("{n} ");
    }
    println!();
    // numbers is NOT usable here — it was consumed by the for loop.
    // Uncommenting the next line would be a compile error:
    // println!("{}", numbers.len());

    // --- for over a slice ---
    // Slices are borrowed views; iterating is always by reference.
    let scores = [95, 82, 73, 64]; // array, not Vec
    for score in &scores {
        print!("{score} ");
    }
    println!();
    // scores is still valid.

    // --- enumerate: get index and value ---
    // `.enumerate()` yields `(usize, &T)` tuples.
    let fruits = ["apple", "banana", "cherry"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{index}: {fruit}");
    }

    // --- for with pattern destructuring ---
    // We can destructure complex elements directly in the for loop.
    let pairs = [(1, "one"), (2, "two"), (3, "three")];
    for (num, word) in &pairs {
        println!("{num} is spelled \"{word}\"");
    }
}
