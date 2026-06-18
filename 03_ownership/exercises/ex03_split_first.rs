// Split array into first element + rest using slices.
// Given &[i32], return a tuple (Option<&i32>, &[i32]).
// The first element (if any) and the remaining slice — both borrowed, not owned.

fn main() {
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let slice: &[i32] = &arr[..];

    let (first, rest): (Option<&i32>, &[i32]) = split_first(slice);

    match first {
        Some(val) => println!("First: {}", val),
        None => println!("Empty slice"),
    }
    println!("Rest: {:?}", rest);

    // The array is still owned by `arr`
    println!("Original array: {:?}", arr);

    // Verify correctness
    assert_eq!(*first.unwrap(), 10);
    assert_eq!(rest, &[20, 30, 40, 50]);

    // Edge case: single element
    let single: [i32; 1] = [99];
    let (f, r) = split_first(&single[..]);
    assert_eq!(*f.unwrap(), 99);
    assert!(r.is_empty());

    // Edge case: empty slice
    let empty: [i32; 0] = [];
    let (f, r) = split_first(&empty[..]);
    assert!(f.is_none());
    assert!(r.is_empty());

    println!("✓ ex03_split_first passed!");
}

// TODO: Implement split_first.
// Given a slice, return (first element if any, remaining slice).
// Hint: use slice.split_first() — or do it manually with slice patterns.
// We want you to implement it MANUALLY (no calling .split_first()).
fn split_first(slice: &[i32]) -> (Option<&i32>, &[i32]) {
    // Your code here
    // Check if slice is empty. If not, split into first element and rest.
    todo!("implement split_first")
}
