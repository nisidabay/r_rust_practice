// BONUS: Fix the ownership errors in this code.
// There were 4 bugs — now fixed.

fn main() {
    let mut s: String = String::from("hello");

    // FIX 1: Pass a REFERENCE &s instead of moving s
    print_length(&s);

    // s is still valid because we only borrowed it
    println!("Original string was: {}", s);

    // FIX 2: Drop the immutable reference before creating mutable one
    // (fixed by removing the immutable ref entirely, since we don't need it)
    // FIX 3: Make s mutable with `let mut s`
    let r2: &mut String = &mut s;
    r2.push_str(" world");
    // r2's mutable borrow ends here
    println!("r2: {}", r2);

    // FIX 4: Change process_vec to take a reference & return cloned data
    let v: Vec<i32> = vec![1, 2, 3];
    let doubled: Vec<i32> = process_vec(&v);
    println!("Doubled: {:?}", doubled);
    println!("Original: {:?}", v); // v is still valid

    println!("✓ BONUS ex04_ownership_debug passed! All 4 bugs fixed.");
}

// Changed to take &String instead of String — borrows, doesn't own
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

// Changed to take &Vec<i32> — borrows instead of owning
fn process_vec(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|x| x * 2).collect()
}
