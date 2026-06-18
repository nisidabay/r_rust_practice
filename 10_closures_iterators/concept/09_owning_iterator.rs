// 09_owning_iterator.rs — into_iter() vs iter() vs iter_mut()
//
// Rust provides three ways to iterate over a collection, each with
// different ownership semantics:
//   iter()      — yields &T (shared references), doesn't consume
//   iter_mut()  — yields &mut T (mutable references), doesn't consume
//   into_iter() — yields T (owned values), consumes the collection

fn main() {
    // --- 1. iter() — borrows immutably ---
    let numbers = vec![10, 20, 30];

    // .iter() yields &i32 — we can use the vector afterwards
    for n in numbers.iter() {
        println!("iter: {n}"); // n is &i32
    }
    println!("Still own numbers: {:?}", numbers); // OK

    // --- 2. iter_mut() — borrows mutably ---
    let mut numbers_mut = vec![1, 2, 3];

    for n in numbers_mut.iter_mut() {
        *n *= 10; // n is &mut i32
    }
    println!("After iter_mut: {:?}", numbers_mut); // OK, we still own it

    // --- 3. into_iter() — consumes the collection ---
    let owned_numbers = vec![100, 200, 300];

    // .into_iter() yields i32 (owned) — vector is consumed
    for n in owned_numbers.into_iter() {
        println!("into_iter: {n}"); // n is i32 (owned)
    }
    // println!("{:?}", owned_numbers); // ERROR: value used after move

    // --- 4. for x in collection desugars to into_iter() ---
    let words = vec!["alpha", "beta", "gamma"];

    // This is equivalent to words.into_iter()
    for w in words {
        println!("for-in consumes: {w}");
    }
    // println!("{:?}", words); // ERROR: moved

    // --- 5. for x in &collection desugars to iter() ---
    let colors = vec!["red", "green", "blue"];

    for c in &colors {
        println!("for &collection: {c}"); // c is &&str, auto-deref
    }
    println!("Still have colors: {:?}", colors);

    // --- 6. for x in &mut collection desugars to iter_mut() ---
    let mut counts = vec![0, 0, 0];

    for c in &mut counts {
        *c += 1;
    }
    println!("After &mut for: {:?}", counts);

    // --- 7. Practical: iter().map() leaves original intact ---
    let scores = vec![85, 92, 78, 90];
    let bonus: Vec<i32> = scores.iter().map(|s| s + 5).collect();
    println!("Original: {:?}, Bonus: {:?}", scores, bonus);

    // --- 8. into_iter() with adapter chain (consumes) ---
    let values = vec![3, 1, 4, 1, 5];
    let sorted_desc: Vec<i32> = {
        let mut v: Vec<i32> = values.into_iter().collect();
        v.sort_by(|a, b| b.cmp(a));
        v
    };
    // println!("{:?}", values); // ERROR: consumed
    println!("Sorted desc: {:?}", sorted_desc);

    // --- 9. Drain — empties a collection without dropping it ---
    let mut drain_me = vec!['a', 'b', 'c', 'd', 'e'];
    let drained: Vec<char> = drain_me.drain(1..3).collect();
    println!("Drained: {:?}, Remaining: {:?}", drained, drain_me);

    println!("\nAll iterator ownership modes demonstrated.");
}
