// Copy types vs move semantics: simple values (integers, floats, bools, char)
// implement the Copy trait and are duplicated automatically. Heap types like
// String and Vec move ownership — the source is invalidated.

fn main() {
    // === COPY TYPES (stack-only, implement Copy trait) ===

    // i32: Copy — x is still valid after assignment
    let x: i32 = 10;
    let y: i32 = x; // x is bitwise-copied
    println!("1. x={}, y={} — both valid, i32 is Copy", x, y);

    // f64: Copy
    let a: f64 = 3.14;
    let b: f64 = a; // full copy
    println!("2. a={}, b={} — f64 is Copy", a, b);

    // bool: Copy
    let flag: bool = true;
    let copy_flag: bool = flag;
    println!("3. flag={}, copy={} — bool is Copy", flag, copy_flag);

    // char: Copy (char is 4 bytes in Rust, always Unicode scalar value)
    let ch: char = 'R';
    let ch2: char = ch;
    println!("4. ch={}, ch2={} — char is Copy", ch, ch2);

    // Tuples of Copy types are also Copy
    let pair: (i32, f64) = (5, 2.5);
    let pair2 = pair; // Copy because both elements are Copy
    println!("5. ({}, {}) and ({}, {}) — tuple of Copy is Copy", pair.0, pair.1, pair2.0, pair2.1);

    // === MOVE SEMANTICS (heap-allocated types) ===

    // String: NOT Copy — ownership moves, source is invalidated
    let s1: String = String::from("hello");
    let s2: String = s1; // s1 MOVES into s2
    // println!("{}", s1); // COMPILE ERROR: value borrowed here after move
    println!("6. s2={} — s1 was moved, s1 no longer valid", s2);

    // Vec<i32>: NOT Copy — heap allocated, moves ownership
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1; // v1 moves into v2
    // println!("{:?}", v1); // COMPILE ERROR: used after move
    println!("7. v2={:?} — v1 moved into v2", v2);

    // String slice (&str): this is a REFERENCE, not an owned value
    // References are Copy — they can be duplicated freely
    let greeting: &str = "hello";
    let greeting2: &str = greeting; // &str is Copy (it's just a fat pointer)
    println!("8. &str: {} and {} — references are Copy", greeting, greeting2);

    // .clone() explicitly duplicates heap data (expensive — use sparingly)
    let name1: String = String::from("Alice");
    let name2: String = name1.clone(); // deep copy of heap data
    println!("9. After clone: name1={}, name2={} — both valid", name1, name2);

    // Key insight: check if a type is Copy by asking:
    // "Is its size known at compile time and does it live entirely on the stack?"
    // If yes → it's likely Copy. If it owns heap data → it moves.
    println!("10. ✓ Copy vs Move demonstrated");
}
