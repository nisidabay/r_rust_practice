// 05_tuple_array — fixed-length compound types
// Tuples group values of possibly different types.
// Arrays group values of the same type.
// Both have a fixed length known at compile time.

fn main() {
    // ----- tuples -----
    // Tuple syntax: (T1, T2, T3, ...)
    let tup: (i32, f64, char) = (42, 3.14, 'R');

    // Destructuring — unpack tuple into individual bindings.
    let (x, y, z) = tup;
    println!("destructured: x={x}, y={y}, z={z}");

    // Index access with dot syntax (tuple uses . not []).
    let first = tup.0;
    let second = tup.1;
    println!("tuple indexing: tup.0={first}, tup.1={second}");

    // A single-element tuple needs a trailing comma: (42,)
    // Without the comma, (42) is just a parenthesized i32.
    let single = (42,);     // tuple
    let not_tuple = 42;     // just i32, parenthesized
    println!("single tuple: {:?}, not tuple: {not_tuple}", single);

    // ----- arrays -----
    // Array syntax: [T; N] — type and length.
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Index with [] (runtime bounds-checked).
    println!("arr[0]={}, arr[2]={}", arr[0], arr[2]);

    // Shorthand: [initial_value; count]
    let ones = [1; 3];      // [1, 1, 1]
    println!("ones: {:?}", ones);

    // Array length at compile time — use .len()
    println!("arr has {} elements", arr.len());

    // Bounds-checking: arr[10] would panic at runtime.
    // Indexing is always checked — no buffer overruns in safe Rust.

    // ----- tuple-array distinction -----
    // Tuples: heterogeneous, use .0 access, optional std::fmt::Debug
    // Arrays: homogeneous, use [i] access, always Debug up to size 32
}
