// Ownership transfer through function calls: passing a value to a function
// moves ownership to the function parameter. The original variable is invalidated.
// Copy types are automatically duplicated instead.

fn main() {
    // === MOVING into functions ===

    let s: String = String::from("hello");
    take_ownership(s); // s is MOVED into the function
    // println!("{}", s); // COMPILE ERROR: s no longer owns the data

    let x: i32 = 42;
    copy_then_double(x); // x is COPIED (i32 is Copy), still valid
    println!("2. x is still valid after copy fn: {}", x);

    // === Returning ownership from functions ===

    let s1: String = give_ownership(); // function creates and returns a String
    println!("3. s1 owns: {}", s1);

    let s2: String = take_and_give_back(s1); // s1 moves in, ownership returned as s2
    // println!("{}", s1); // COMPILE ERROR: s1 was moved into the function
    println!("4. s2 owns after take_and_give_back: {}", s2);

    // === The tuple pattern: return multiple values ===

    let original: String = String::from("rust");
    let (len, returned) = calculate_length(original);
    // println!("{}", original); // COMPILE ERROR: moved into function
    println!(
        "5. tuple pattern: returned='{}', length={}",
        returned, len
    );

    // === Why this is painful without references ===
    // Every function that needs to use a heap value must give it back
    // or the caller loses access. This is why we need REFERENCES (next concept).
    println!("6. ✓ Ownership transfer through functions demonstrated");
}

// Takes ownership of a String, then drops it when scope ends
fn take_ownership(s: String) {
    println!("1. Inside take_ownership: {}", s);
} // s is dropped here — heap memory freed

// Takes a Copy type — caller keeps the value
fn copy_then_double(n: i32) {
    let doubled: i32 = n * 2;
    println!("   Inside copy_then_double: {} doubled = {}", n, doubled);
} // n is on the stack, nothing special happens

// Creates a String and returns it — ownership transfers to caller
fn give_ownership() -> String {
    let result: String = String::from("hello from function");
    result // ownership moves to the caller
}

// Takes ownership and returns it back — "borrowing by moving back"
fn take_and_give_back(s: String) -> String {
    println!("   Inside take_and_give_back: got '{}'", s);
    s // ownership returned to caller
}

// Takes ownership, computes something, returns both the String and a computed value
fn calculate_length(s: String) -> (usize, String) {
    let length: usize = s.len();
    (length, s) // return tuple: computed value + ownership back
}
