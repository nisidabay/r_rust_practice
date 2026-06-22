/*
 * 01_immutable_ref.rs — Practical Rust
 *
 * Question: How do I look at data without owning it?
 *
 * &T = immutable reference (borrow). You can READ through it.
 * Multiple readers allowed — many immutable references to same data.
 * The borrow ends when the reference goes out of scope (not the data).
 * Dereference with * to access the value.
 */

fn main() {
    let s = String::from("hello");

    // Borrow s (immutably)
    let r1: &String = &s;
    let r2: &String = &s; // multiple readers allowed!

    println!("r1: {}, r2: {}", r1, r2); // reading through references

    // Dereferencing with *
    let x = 42;
    let rx: &i32 = &x;
    println!("x = {}, *rx = {}", x, *rx);

    // References are automatic — Rust dereferences for common ops
    let len = s.len();
    let rlen = r1.len(); // auto-dereference: (&String).len() calls String::len()
    println!("len: {}, rlen: {}", len, rlen);

    // Passing &T to a function
    let numbers = vec![10, 20, 30];
    print_first(&numbers);
    println!("still have numbers: {:?}", numbers); // we only borrowed
}

fn print_first(v: &Vec<i32>) {
    if let Some(first) = v.first() {
        println!("first element: {}", first);
    }
}
