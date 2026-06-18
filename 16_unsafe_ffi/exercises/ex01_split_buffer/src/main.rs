// ex01: Implement split_at_mut using raw pointers
//
// std::slice::split_at_mut splits a mutable slice into two at an index.
// Internally, it uses unsafe raw pointer arithmetic.
// Your task: implement your own version using raw pointers.

/// Split a mutable slice into two parts at `mid`.
/// Returns (left, right) where left contains [0..mid) and right [mid..len).
///
/// # Safety
/// `mid` must be <= `buf.len()`. Panics otherwise (like the std version).
fn split_at_mut<T>(buf: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = buf.len();
    assert!(mid <= len, "mid > len");

    // Get a raw pointer to the start of the buffer
    let ptr: *mut T = buf.as_mut_ptr();

    unsafe {
        // The left slice is from ptr with length mid
        // The right slice is from ptr.add(mid) with length len - mid
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    println!("=== Exercise 01: split_at_mut with raw pointers ===");

    let mut data = [1, 2, 3, 4, 5, 6];
    println!("Original: {:?}", data);

    let (left, right) = split_at_mut(&mut data, 3);
    println!("Left:  {:?}", left);
    println!("Right: {:?}", right);

    // Verify they point to the same memory
    left[0] = 99;
    right[0] = 42;
    println!("After mutation through split references: {:?}", data);
    assert_eq!(data, [99, 2, 3, 42, 5, 6]);

    // Edge cases
    let (empty_left, rest) = split_at_mut(&mut data, 0);
    assert!(empty_left.is_empty());
    assert_eq!(rest.len(), 6);

    let (all, empty_right) = split_at_mut(&mut data, 6);
    assert_eq!(all.len(), 6);
    assert!(empty_right.is_empty());

    println!("All assertions passed! ✓");

    println!();
    println!("How it works:");
    println!("  1. Get a raw *mut T pointer via buf.as_mut_ptr()");
    println!("  2. Use from_raw_parts_mut(ptr, len) for the left half");
    println!("  3. Use from_raw_parts_mut(ptr.add(mid), len-mid) for the right half");
    println!("  4. This is exactly what std's split_at_mut does internally");
}
