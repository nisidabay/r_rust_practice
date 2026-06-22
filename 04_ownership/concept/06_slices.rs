/*
 * 06_slices.rs — Practical Rust
 *
 * Question: How do I reference part of a String or array without copying?
 *
 * A slice is a reference to a contiguous sequence of elements.
 * &str = string slice (reference to part of a String or &str literal)
 * &[i32] = array/vector slice
 * Slices are DSTs (dynamically sized) — they exist as references only.
 * Slice range syntax: &s[0..5] (half-open), &s[0..=5] (inclusive)
 */

fn main() {
    // String slices (&str)
    let s = String::from("hello world");

    let hello = &s[0..5]; // chars 0..4 (exclusive end)
    let world = &s[6..11]; // chars 6..10

    println!("hello = '{}', world = '{}'", hello, world);

    // Shortcut: from start
    let from_start = &s[..5]; // same as &s[0..5]
    println!("from_start = '{}'", from_start);

    // Shortcut: to end
    let to_end = &s[6..]; // same as &s[6..len]
    println!("to_end = '{}'", to_end);

    // Shortcut: the whole thing
    let whole = &s[..]; // same as &s[0..len]
    println!("whole = '{}'", whole);

    // String literals are &str
    let lit: &str = "hello";
    println!("literal: {}", lit);

    // Array/vector slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; // [2, 3, 4]
    println!("arr slice: {:?}", slice);

    // Vec slices
    let v = vec![10, 20, 30, 40, 50];
    let first_two = &v[0..2];
    println!("first two: {:?}", first_two);

    // Slice bounds checking — panics at runtime if out of bounds
    let safe_slice = &v.get(0..3).unwrap_or(&v[..0]);
    println!("safe: {:?}", safe_slice);

    // Mutable slices
    let mut data = vec![1, 2, 3, 4, 5];
    let mid = &mut data[1..4]; // mutable slice of elements [2,3,4]
    for x in mid.iter_mut() {
        *x *= 10;
    }
    println!("after mutating slice: {:?}", data);
}
