// 06_iterator_adapters — map, filter, take, skip, chain
//
// Iterator adapters are methods that transform one iterator into another.
// They are lazy — they don't do anything until a consuming adapter is called.
// This is the heart of Rust's expressive functional-style data processing.

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // --- 1. map — transform each element ---
    let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect();
    println!("map (*2): {:?}", doubled);

    // --- 2. filter — keep elements matching a predicate ---
    let evens: Vec<&i32> = data.iter().filter(|x| *x % 2 == 0).collect();
    println!("filter (evens): {:?}", evens);

    // --- 3. take — take only the first N elements ---
    let first_three: Vec<&i32> = data.iter().take(3).collect();
    println!("take(3): {:?}", first_three);

    // --- 4. skip — skip the first N elements ---
    let after_five: Vec<&i32> = data.iter().skip(5).collect();
    println!("skip(5): {:?}", after_five);

    // --- 5. chain — concatenate two iterators ---
    let first = vec![1, 2, 3];
    let second = vec![4, 5, 6];
    let chained: Vec<i32> = first.into_iter().chain(second.into_iter()).collect();
    println!("chain: {:?}", chained);

    // --- 6. step_by — take every nth element ---
    let every_third: Vec<&i32> = data.iter().step_by(3).collect();
    println!("step_by(3): {:?}", every_third);

    // --- 7. enumerate — add an index ---
    let enumerated: Vec<(usize, &i32)> = data.iter().enumerate().collect();
    println!("enumerate: {:?}", enumerated);

    // --- 8. rev — reverse the iterator ---
    let reversed: Vec<&i32> = data.iter().rev().collect();
    println!("rev: {:?}", reversed);

    // --- 9. cloned — clone each element (from &T to T) ---
    let cloned: Vec<i32> = data.iter().cloned().collect();
    println!("cloned: {:?}", cloned);

    // --- 10. cycle — repeat the iterator forever ---
    let repeated: Vec<i32> = vec![1, 2, 3].iter().cycle().take(10).cloned().collect();
    println!("cycle take(10): {:?}", repeated);

    // --- 11. Practical: take_while and skip_while ---
    let prefix: Vec<&i32> = data.iter().take_while(|x| **x < 5).collect();
    println!("take_while (<5): {:?}", prefix);

    let after: Vec<&i32> = data.iter().skip_while(|x| **x < 5).collect();
    println!("skip_while (<5): {:?}", after);

    println!("\nAll iterator adapters demonstrated.");
}
