// Split array into first element + rest using slices.

fn main() {
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let slice: &[i32] = &arr[..];

    let (first, rest): (Option<&i32>, &[i32]) = split_first(slice);

    match first {
        Some(val) => println!("First: {}", val),
        None => println!("Empty slice"),
    }
    println!("Rest: {:?}", rest);

    println!("Original array: {:?}", arr);

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

// Manual implementation: check empty, split into first + rest
fn split_first(slice: &[i32]) -> (Option<&i32>, &[i32]) {
    if slice.is_empty() {
        (None, &[])
    } else {
        (Some(&slice[0]), &slice[1..])
    }
}
