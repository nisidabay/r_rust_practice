// 01_generic_fn.rs
// Generic functions — writing code that works with *any* type.
//
// Generic functions allow a single definition to operate on many types.
// The type parameter <T> is a placeholder that gets filled in at compile time
// (monomorphization — each concrete type generates its own optimized copy).
//
// Run: rustc --edition 2021 01_generic_fn.rs && ./01_generic_fn

/// Returns the largest element in a slice, or None if the slice is empty.
/// T must implement PartialOrd (so we can compare) and Copy (so we can return
/// an owned value without borrowing).
fn largest<T: PartialOrd + Copy>(slice: &[T]) -> Option<T> {
    if slice.is_empty() {
        return None;
    }
    let mut biggest = slice[0];
    for &item in slice.iter() {
        if item > biggest {
            biggest = item;
        }
    }
    Some(biggest)
}

/// Returns both the smallest and largest element in a slice.
/// Demonstrates multiple type parameters (though here they're the same type T,
/// the syntax extends naturally to <T, U> for different types).
fn min_max<T: PartialOrd + Copy>(slice: &[T]) -> Option<(T, T)> {
    if slice.is_empty() {
        return None;
    }
    let mut min = slice[0];
    let mut max = slice[0];
    for &item in slice.iter() {
        if item < min {
            min = item;
        }
        if item > max {
            max = item;
        }
    }
    Some((min, max))
}

/// Swaps two values of the same type. Demonstrates that T can be inferred
/// from the arguments — no explicit type annotation needed at the call site.
fn swap<T>(a: &mut T, b: &mut T) {
    // std::mem::swap is the idiomatic way, but we implement by hand to show
    // the pattern. (In real code, just use mem::swap.)
    // Note: T does NOT need Copy here because we move out and back in.
    unsafe {
        // Safety: We're swapping two valid, non-overlapping values.
        let tmp = std::ptr::read(a);
        std::ptr::write(a, std::ptr::read(b));
        std::ptr::write(b, tmp);
    }
}

fn main() {
    // --- largest with integers ---
    let nums = vec![3, 7, 2, 9, 1, 8];
    match largest(&nums) {
        Some(max) => println!("largest int: {max}"),
        None => println!("empty slice"),
    }

    // --- largest with floats ---
    let floats = vec![2.5, 1.1, 9.8, 3.3];
    match largest(&floats) {
        Some(max) => println!("largest float: {max}"),
        None => println!("empty slice"),
    }

    // --- largest with chars ---
    let chars = vec!['a', 'z', 'm', 'q'];
    match largest(&chars) {
        Some(max) => println!("largest char: {max}"),
        None => println!("empty slice"),
    }

    // --- empty slice ---
    let empty: [i32; 0] = [];
    match largest(&empty) {
        Some(_) => println!("should not happen"),
        None => println!("largest of empty: None (correct)"),
    }

    // --- min_max ---
    let data = [10, -5, 42, 0, 33];
    if let Some((mn, mx)) = min_max(&data) {
        println!("min_max: min={mn}, max={mx}");
    }

    // --- swap ---
    let mut x = 100;
    let mut y = 999;
    println!("before swap: x={x}, y={y}");
    swap(&mut x, &mut y);
    println!("after swap:  x={x}, y={y}");

    // --- swap works with strings too ---
    let mut a = "hello";
    let mut b = "world";
    println!("before swap: a={a}, b={b}");
    swap(&mut a, &mut b);
    println!("after swap:  a={a}, b={b}");
}
