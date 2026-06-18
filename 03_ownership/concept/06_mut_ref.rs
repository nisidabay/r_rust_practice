// Mutable references (&mut T) let you modify borrowed data.
// The key rule: only ONE mutable reference at a time.
// This prevents data races at compile time — Rust's superpower.

fn main() {
    // === BASIC MUTABLE REFERENCE ===

    let mut s: String = String::from("hello");

    // &mut s creates a mutable reference
    change(&mut s);
    println!("1. After change: {}", s);

    // === ONE MUTABLE REFERENCE RULE ===
    // You CANNOT have both a mutable and immutable reference to the same data

    let data: String = String::from("count me");

    // UNCOMMENTING THESE LINES WOULD CAUSE A COMPILE ERROR:
    // let r1: &String = &data;     // immutable borrow
    // let r2: &mut String = &mut data; // can't borrow as mutable
    // println!("{}, {}", r1, r2);
    // The compiler catches: cannot borrow `data` as mutable because
    // it is also borrowed as immutable
    let _ = data; // suppress unused warning

    // But you CAN have a mutable reference if no immutable references exist

    let mut text: String = String::from("hello");

    let r_mut: &mut String = &mut text;
    r_mut.push_str(" world");
    println!("2. After mutable operation: {}", r_mut);
    // r_mut goes out of scope here — the mutable borrow ends

    // Now we can have immutable references again
    let r_immut: &String = &text;
    println!("3. After mutable borrow ended: {}", r_immut);

    // === SCOPE-BASED BORROW SEPARATION ===
    // Use blocks to limit mutable borrow scope

    let mut name: String = String::from("Alice");

    {
        let ref_mut: &mut String = &mut name;
        ref_mut.push_str(" Smith");
    } // mutable borrow ends here

    // Now immutable references are fine
    println!("4. After scoped mutable change: {}", name);

    // === MUTABLE REFERENCES WITH INTEGERS ===

    let mut counter: i32 = 0;
    let c: &mut i32 = &mut counter;
    *c += 1;
    *c += 2;
    // counter is now 3
    // println!("{}", counter); // COMPILE ERROR: can't read while mutably borrowed
    println!("5. *c = {}", c);
    // c goes out of scope here
    println!("6. counter = {} — back to normal", counter);

    // === MUTABLE REFERENCE RESTRICTIONS ===
    // Cannot have &mut T while any &T exists

    let mut value: String = String::from("test");
    let imm: &String = &value; // immutable borrow starts
    println!("7. Immutable read: {}", imm);
    // imm goes out of scope here — borrow ends

    let mut_ref: &mut String = &mut value; // now mutable borrow is OK
    mut_ref.push_str("ing");
    println!("8. After mutable: {}", mut_ref);

    println!("9. ✓ Mutable references demonstrated");
}

// &mut String means "mutable reference to a String" — can modify the original
fn change(s: &mut String) {
    s.push_str(" world"); // modifies the String through the mutable reference
}
