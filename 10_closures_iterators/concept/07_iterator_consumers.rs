// 07_iterator_consumers — collect, sum, count, fold, for_each
//
// Consumer adapters drive the iterator by calling next() repeatedly.
// Without a consumer, adapter chains are lazy and do nothing.
// Collect is the most common consumer for building collections.

fn main() {
    let numbers = vec![2, 4, 6, 8, 10];

    // --- 1. collect — build a collection from an iterator ---
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("collect Vec: {:?}", doubled);

    // Collect into other collection types
    let set: std::collections::HashSet<i32> = numbers.iter().cloned().collect();
    println!("collect HashSet: {:?}", set);

    // Collect into String (requires chars or &str)
    let chars = vec!['h', 'e', 'l', 'l', 'o'];
    let s: String = chars.iter().collect();
    println!("collect String: {s}");

    // Collect into Result (collects first Err or Ok)
    let results = vec![Ok(1), Ok(2), Err("oops"), Ok(4)];
    // let collected: Result<Vec<i32>, &str> = results.into_iter().collect();
    // println!("collect Result: {:?}", collected);

    // --- 2. sum — sum numeric elements ---
    let total: i32 = numbers.iter().sum();
    println!("sum: {total}");

    // --- 3. product — multiply all elements ---
    let product: i32 = numbers.iter().product();
    println!("product: {product}");

    // --- 4. count — number of elements ---
    let cnt = numbers.iter().count();
    println!("count: {cnt}");

    // --- 5. fold — accumulate with an initial value and closure ---
    // fold(init, |acc, x| ...) is the most general consumer
    let sum_fold = numbers.iter().fold(0, |acc, x| acc + x);
    println!("fold sum: {sum_fold}");

    let product_fold = numbers.iter().fold(1, |acc, x| acc * x);
    println!("fold product: {product_fold}");

    let max_fold = numbers.iter().fold(i32::MIN, |acc, x| acc.max(*x));
    println!("fold max: {max_fold}");

    // --- 6. reduce — like fold but uses first element as initial value ---
    let sum_reduce = numbers.iter().cloned().reduce(|acc, x| acc + x);
    println!("reduce sum: {:?}", sum_reduce);

    // --- 7. for_each — run a side-effect closure for each element ---
    numbers.iter().for_each(|x| print!("{x} "));
    println!(); // newline

    // --- 8. all, any — test boolean conditions ---
    let all_even = numbers.iter().all(|x| x % 2 == 0);
    let any_greater_than_5 = numbers.iter().any(|x| *x > 5);
    println!("all even: {all_even}, any > 5: {any_greater_than_5}");

    // --- 9. find, position — locate an element ---
    let found = numbers.iter().find(|&&x| x == 6);
    println!("find 6: {:?}", found);

    let pos = numbers.iter().position(|&x| x == 8);
    println!("position of 8: {:?}", pos);

    // --- 10. min, max ---
    let min_val = numbers.iter().min();
    let max_val = numbers.iter().max();
    println!("min: {:?}, max: {:?}", min_val, max_val);

    println!("\nAll iterator consumers demonstrated.");
}
