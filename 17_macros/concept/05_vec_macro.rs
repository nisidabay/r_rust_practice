// 05_vec_macro.rs — Re-implement vec![] to understand how it works
//
// The standard vec![] macro does three things:
//  1. vec![x; n]  — repeat expression x, n times
//  2. vec![a, b, c] — literal list
//  3. vec![] — empty vec
//
// We'll build our own to see how the compiler expands these.

/// Our own version of vec![].
/// Features:
///   - vec![x; n] — repeat
///   - vec![a, b, c] — list
///   - vec![a, b, c,] — trailing comma
///   - vec![] — empty
macro_rules! my_vec {
    // Repeat pattern: [expr; count]
    ($value:expr; $count:expr) => {{
        let count = $count;
        let mut v = Vec::with_capacity(count);
        for _ in 0..count {
            v.push($value);
        }
        v
    }};

    // List with at least one element, optional trailing comma
    ($($x:expr),+ $(,)?) => {{
        let mut v = Vec::new();
        $(
            v.push($x);
        )+
        v
    }};

    // Empty vec
    () => {
        Vec::new()
    };
}

/// Demonstrate how the actual vec![] uses unsafe for performance.
/// The standard library uses ptr::write to avoid the overhead of push()
/// and to avoid intermediate length checks. Here's a closer approximation:
macro_rules! efficient_vec {
    // Repeat pattern
    ($value:expr; $count:expr) => {{
        let count = $count;
        let mut v: Vec<_> = (0..count).map(|_| $value).collect();
        v
    }};

    // List
    ($($x:expr),+ $(,)?) => {{
        let mut v = Vec::new();
        v.reserve_exact($crate::macros::count![$($x),+]);
        // In real std: unsafe { ptr::write } — here we just push
        $(
            v.push($x);
        )+
        v
    }};

    () => {
        Vec::new()
    };
}

/// Helper macro to count repetitions at compile time.
/// Used by efficient_vec if we wanted.
macro_rules! count {
    () => (0usize);
    ($x:expr $(,)?) => (1usize);
    ($x:expr, $($rest:expr),+ $(,)?) => (1usize + count!($($rest),+));
}

fn main() {
    println!("=== Re-implementing vec![] ===");

    // List syntax
    let v1 = my_vec![1, 2, 3, 4, 5];
    println!("my_vec![1, 2, 3, 4, 5] = {:?}", v1);

    // Repeat syntax
    let v2 = my_vec![0; 10];
    println!("my_vec![0; 10] = {:?}", v2);

    // Empty
    let v3: Vec<i32> = my_vec![];
    println!("my_vec![] = {:?}", v3);

    // Trailing comma
    let v4 = my_vec!["a", "b", "c",];
    println!("my_vec![\"a\", \"b\", \"c\",] = {:?}", v4);

    // Count helper
    println!("count![1, 2, 3] = {}", count![1, 2, 3]);
    println!("count![] = {}", count![]);

    // Compare with real vec! (same output)
    let real = vec![1, 2, 3, 4, 5];
    println!("Actual vec! output: {:?}", real);

    println!("\nHow vec![] works:");
    println!("- vec![x; n]: Creates Vec with capacity n, fills via loop");
    println!("- vec![a, b, c]: Infallible — unsafe ptr::write avoids bound checks");
    println!("- vec![]: Empty Vec");
    println!("The real std vec![] uses Box<[T]> internally for the infallible path.");
}
