// 06_range — range expressions: .. and ..=
// Range is a half-open interval [start, end) for `..`
// and inclusive [start, end] for `..=`.
// Ranges are iterable and work with for loops.

fn main() {
    // Exclusive range: start..end (end is NOT included).
    // 0..5 produces values 0, 1, 2, 3, 4.
    println!("0..5 (exclusive):");
    for i in 0..5 {
        print!("{i} ");
    }
    println!();

    // Inclusive range: start..=end (end IS included).
    // 0..=5 produces values 0, 1, 2, 3, 4, 5.
    println!("0..=5 (inclusive):");
    for i in 0..=5 {
        print!("{i} ");
    }
    println!();

    // Reverse iteration: call .rev() on a range.
    println!("5..=1 reversed:");
    for i in (1..=5).rev() {
        print!("{i} ");
    }
    println!();

    // Ranges are also used for slicing arrays (borrowing a subset).
    let arr = [10, 20, 30, 40, 50];
    let slice = &arr[1..4]; // elements at index 1, 2, 3
    println!("arr[1..4] = {:?}", slice);

    // Range is a type: std::ops::Range<T> and std::ops::RangeInclusive<T>.
    let r: std::ops::Range<i32> = 0..10;
    println!("range length = {}", r.len());

    // Step by other than 1 requires .step_by()
    for i in (0..10).step_by(2) {
        print!("{i} ");
    }
    println!("(step by 2)");
}
