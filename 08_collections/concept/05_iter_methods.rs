fn main() {
    // Iterator methods transform collections without manual loops.
    // These are Rust's answer to Python's map/filter/reduce.

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // --- .iter() — borrow each element ---
    // Without .iter(), a for loop would consume the Vec. .iter() borrows.
    for n in numbers.iter() {
        print!("{} ", n);
    }
    println!("  (after iter, numbers is still usable)");

    // --- .map() — transform each element (lazy, needs .collect()) ---
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("map: {:?}", doubled);

    // --- .filter() — keep elements matching a predicate ---
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("filter: {:?}", evens);

    // --- Chaining: map + filter + collect ---
    // Common pattern: filter THEN map (avoids wasted work)
    let result: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)        // keep evens
        .map(|x| x * 10)                 // multiply by 10
        .collect();
    println!("chain: {:?}", result);     // [20, 40, 60, 80, 100]

    // --- .fold() — accumulate (like Python's reduce) ---
    // Start with 0, add each element
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("fold sum: {}", sum);        // 55

    // Build a string with fold
    let labels = numbers.iter().fold(String::new(), |acc, x| {
        if acc.is_empty() { format!("{}", x) }
        else { format!("{}, {}", acc, x) }
    });
    println!("fold string: '{}'", labels);

    // --- .sum() — convenience for numeric fold ---
    let total: i32 = numbers.iter().sum();
    println!("sum: {}", total);           // 55

    // --- Any / All — boolean checks ---
    let has_big = numbers.iter().any(|x| *x > 8);
    let all_small = numbers.iter().all(|x| *x < 100);
    println!("any > 8: {}, all < 100: {}", has_big, all_small);
}
