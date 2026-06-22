// 06_iterator_consumers.rs — Iterator consumers that DRIVE the iterator
//
// Unlike adapters (which are lazy), consumers are EAGER — they call next()
// until the iterator is exhausted and produce a final value.

fn main() {
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];

    // collect: gather into a collection
    let collected: Vec<i32> = nums.iter().filter(|x| *x % 2 == 0).copied().collect();
    println!("collect (evens): {:?}", collected);

    // for_each: run a side effect for each element
    nums.iter().for_each(|x| print!("{} ", x));
    println!(); // newline

    // fold: accumulate with an initial value and closure
    // Like reduce, but you provide the starting value
    let sum = nums.iter().fold(0, |acc, x| acc + x);
    println!("fold sum: {}", sum);

    // fold with string building
    let words = vec!["hello", "world", "from", "rust"];
    let sentence = words.iter().fold(String::new(), |acc, w| {
        if acc.is_empty() { w.to_string() } else { format!("{} {}", acc, w) }
    });
    println!("fold sentence: '{}'", sentence);

    // reduce: combine elements (returns Option, no initial value)
    let max = nums.iter().copied().reduce(|a, b| if a > b { a } else { b });
    println!("reduce max: {:?}", max);

    // product through reduce
    let product = nums.iter().take(4).copied().reduce(|a, b| a * b);
    println!("reduce product (first 4): {:?}", product);

    // any: does any element match?
    let has_large = nums.iter().any(|x| *x > 7);
    println!("any > 7? {}", has_large);

    // all: do ALL elements match?
    let all_positive = nums.iter().all(|x| *x > 0);
    println!("all positive? {}", all_positive);

    // find: find first matching element
    let first_even = nums.iter().find(|x| *x % 2 == 0);
    println!("find first even: {:?}", first_even);

    // position: find index of first match
    let first_even_idx = nums.iter().position(|x| *x % 2 == 0);
    println!("position first even: {:?}", first_even_idx);

    // count: number of elements
    let total = nums.iter().count();
    println!("count: {}", total);

    // max/min on iterator (needs Ord)
    let biggest = nums.iter().max();
    let smallest = nums.iter().min();
    println!("max: {:?}, min: {:?}", biggest, smallest);

    println!("\nIterator consumers: they DRIVE iteration and produce final values.");
}
