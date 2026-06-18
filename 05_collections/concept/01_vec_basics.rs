// 01_vec_basics.rs — Vec::new, vec!, push, pop, indexing, len
//
// A Vec<T> is a growable array — it owns a heap-allocated buffer of consecutive Ts.
// First 5 lines: create, push, pop, index, check length.

fn main() {
    // 1. Vec::new — start empty
    let mut v: Vec<i32> = Vec::new();

    // 2. vec! macro — convenient inline initializer
    let w = vec![10, 20, 30];

    // 3. push — append to the end
    v.push(1);
    v.push(2);
    v.push(3);

    // 4. pop — remove and return the last element (Option<T>)
    let _last = v.pop(); // Some(3)

    // 5. Indexing with [] — panics on out-of-bounds
    let _second = w[1]; // 20

    // 6. len / is_empty
    println!("v has {} elements (empty? {})", v.len(), v.is_empty());
    println!("w has {} elements", w.len());

    // 7. get — safe indexing that returns Option<&T>
    match w.get(5) {
        Some(val) => println!("w[5] = {val}"),
        None => println!("w[5] is out of bounds — no panic!"),
    }

    // 8. contains — linear search
    println!("w contains 20? {}", w.contains(&20));

    // 9. reserve / capacity
    let mut c = Vec::with_capacity(10);
    println!("capacity before push: {}", c.capacity());
    c.push(42);
    println!("capacity after  push: {}", c.capacity()); // still 10
}
