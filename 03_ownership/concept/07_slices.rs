// Slices let you borrow a CONTIGUOUS PIECE of a collection without owning it.
// A slice is a reference to a range of elements — it's a fat pointer (ptr + length).
// String slices &str and array slices &[T] are fundamental to Rust ergonomics.

fn main() {
    // === STRING SLICES (&str) ===

    let s: String = String::from("hello world");

    // Slice from index 0 to 5 (exclusive) — borrows part of the String
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    println!("1. Sliced: '{}' and '{}'", hello, world);

    // Shorthand: from start
    let hello2: &str = &s[..5];
    println!("2. From start: '{}'", hello2);

    // Shorthand: to end
    let world2: &str = &s[6..];
    println!("3. To end: '{}'", world2);

    // Full string slice
    let full: &str = &s[..];
    println!("4. Full slice: '{}'", full);

    // === ARRAY SLICES (&[T]) ===

    let arr: [i32; 5] = [10, 20, 30, 40, 50];

    // Slice from index 1 to 3 (exclusive) — borrows part of the array
    let middle: &[i32] = &arr[1..4];
    println!("5. Array slice [1..4]: {:?}", middle);

    // First three elements
    let first_three: &[i32] = &arr[..3];
    println!("6. First three: {:?}", first_three);

    // Last two elements
    let last_two: &[i32] = &arr[3..];
    println!("7. Last two: {:?}", last_two);

    // === VECTOR SLICES ===

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let slice: &[i32] = &v[1..4];
    println!("8. Vector slice [1..4]: {:?}", slice);

    // === PRACTICAL USE: FIRST WORD FUNCTION ===

    let phrase: String = String::from("hello world rust");
    let first: &str = first_word(&phrase);
    println!("9. First word of '{}' is '{}'", phrase, first);

    // === SLICE BOUNDARIES ===
    // Rust panics at runtime if you slice outside bounds
    // (But the compiler ensures the reference doesn't outlive the data)

    let data: String = String::from("safe");
    let prefix: &str = &data[..2];
    println!("10. Prefix of '{}': '{}'", data, prefix);

    // Slices are a VIEW — they don't own the data
    // The owner (String/array/vec) must outlive all slices into it
    println!("11. ✓ Slices demonstrated");
}

// Returns a string slice into the original `s` — no allocation, no copy
fn first_word(s: &String) -> &str {
    // Iterate over bytes, find first space
    for (i, &byte) in s.as_bytes().iter().enumerate() {
        if byte == b' ' {
            return &s[..i]; // slice from start to the space
        }
    }
    &s[..] // no space found — return full string slice
}
