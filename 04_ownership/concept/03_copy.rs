/*
 * 03_copy.rs — Practical Rust
 *
 * Question: Why doesn't everything just copy? What types are Copy?
 *
 * Copy types are stored entirely on the stack — they're cheap to duplicate.
 * Rust copies them implicitly. Types with heap allocation (String, Vec)
 * are NOT Copy — they implement Clone instead (explicit .clone()).
 *
 * Common Copy types: i32, u32, f64, bool, char, tuples of Copy types.
 * Clone trait: provides .clone() for explicit deep copies.
 */

fn main() {
    // Copy types — implicit copy on assignment
    let a: i32 = 42;
    let b = a; // a is COPIED
    println!("a = {}, b = {}", a, b); // both accessible

    let flag: bool = true;
    let copy_flag = flag;
    println!("flag = {}, copy = {}", flag, copy_flag);

    let ch: char = 'R';
    let ch2 = ch;
    println!("ch = {}, ch2 = {}", ch, ch2);

    let pi: f64 = 3.14159;
    let pi2 = pi;
    println!("pi = {}, pi2 = {}", pi, pi2);

    // Tuples of Copy types are also Copy
    let point = (10, 20);
    let point2 = point;
    println!("point: {:?}, point2: {:?}", point, point2);

    // Clone trait — explicit deep copy for heap-allocated types
    let s1 = String::from("hello");
    let s2 = s1.clone(); // explicit deep copy
    println!("s1 = {}, s2 = {}", s1, s2); // both valid because .clone()

    // Vec clone
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone();
    println!("v1 = {:?}, v2 = {:?}", v1, v2);
}
