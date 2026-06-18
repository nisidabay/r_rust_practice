// BONUS EXERCISE: Fix the ownership errors in this code.
// The compiler will tell you exactly what's wrong — your job is to fix it.
// Read the error messages carefully and apply the right fix.

// This program tries to:
// 1. Create a String
// 2. Pass it to a function
// 3. Use it again after the function call
// 4. Modify it through a mutable reference

// THERE ARE 4 COMPILE ERRORS. Fix all of them.

fn main() {
    let s: String = String::from("hello");

    // BUG 1: s is moved into print_length, can't use s after
    print_length(s);

    // BUG 2: s is used here, but it was moved above
    println!("Original string was: {}", s);

    // BUG 3: tring to get a mutable reference while immutable ref exists
    let r1: &String = &s;
    let r2: &mut String = &mut s;
    r2.push_str(" world");
    println!("r1: {}, r2: {}", r1, r2);

    // BUG 4: use_after_move — v is moved into process_vec
    let v: Vec<i32> = vec![1, 2, 3];
    let doubled: Vec<i32> = process_vec(v);
    println!("Doubled: {:?}", doubled);
    println!("Original: {:?}", v);

    println!("✓ BONUS ex04_ownership_debug passed!");
}

fn print_length(s: String) {
    println!("Length: {}", s.len());
}

fn process_vec(v: Vec<i32>) -> Vec<i32> {
    v.iter().map(|x| x * 2).collect()
}
