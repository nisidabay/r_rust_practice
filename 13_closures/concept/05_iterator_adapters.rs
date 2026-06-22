// 05_iterator_adapters.rs — Iterator adapters: map, filter, take, skip, chain, zip
//
// Iterator adapters TRANSFORM an iterator into another iterator.
// They are LAZY — nothing happens until a consumer (like collect) is called.

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // map: transform each element
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("map (x2): {:?}", doubled);

    // filter: keep elements matching a predicate
    let evens: Vec<&i32> = nums.iter().filter(|x| *x % 2 == 0).collect();
    println!("filter (evens): {:?}", evens);

    // chain: concatenate two iterators
    let first = vec![1, 2, 3];
    let second = vec![4, 5, 6];
    let chained: Vec<i32> = first.iter().chain(second.iter()).copied().collect();
    println!("chain: {:?}", chained);

    // zip: pair elements from two iterators
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![85, 92, 78];
    let paired: Vec<(&str, &i32)> = names.iter().zip(scores.iter()).map(|(a, b)| (*a, b)).collect();
    println!("zip: {:?}", paired);

    // take: limit to first N elements
    let first_three: Vec<i32> = nums.iter().take(3).copied().collect();
    println!("take(3): {:?}", first_three);

    // skip: skip first N elements
    let after_three: Vec<i32> = nums.iter().skip(3).copied().collect();
    println!("skip(3): {:?}", after_three);

    // Combining adapters: chain of transformations
    let result: Vec<i32> = nums.iter()
        .filter(|x| *x % 2 == 0)    // keep evens: 2, 4, 6, 8
        .map(|x| x * 10)             // multiply by 10: 20, 40, 60, 80
        .take(2)                     // first two: 20, 40
        .collect();
    println!("chain (filter>map>take): {:?}", result);

    // zip with range for indexing
    let items = vec!["a", "b", "c", "d"];
    let indexed: Vec<(usize, &&str)> = (0..).zip(items.iter()).take(4).collect();
    println!("zip with index: {:?}", indexed);

    // filter_map: filter and map in one step (returns Option)
    let words = vec!["1", "hello", "42", "world", "99"];
    let parsed: Vec<i32> = words.iter()
        .filter_map(|w| w.parse::<i32>().ok())
        .collect();
    println!("filter_map (parse numbers): {:?}", parsed);

    println!("\nIterator adapters: lazy transformations that compose beautifully.");
}
