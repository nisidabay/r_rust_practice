// 05_iterator_trait — Iterator trait, next(), consuming adapters
//
// The Iterator trait is the heart of iteration in Rust. It has one required
// method: next(&mut self) -> Option<Self::Item>. Everything else is a default
// method built on top of next().

fn main() {
    // --- 1. The Iterator trait (conceptual) ---
    // trait Iterator {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item>;
    //     // many default methods...
    // }

    // --- 2. Creating iterators from collections ---
    let numbers = vec![1, 2, 3, 4, 5];

    // .iter() — yields references (&T)
    let mut iter = numbers.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), None); // exhausted

    // .into_iter() — consumes the collection, yields owned values (T)
    let owned: Vec<i32> = numbers.clone().into_iter().collect();
    println!("into_iter: {:?}", owned);

    // .iter_mut() — yields &mut T
    let mut mutable = vec![10, 20, 30];
    for val in mutable.iter_mut() {
        *val *= 2;
    }
    println!("iter_mut doubled: {:?}", mutable);

    // --- 3. Manual next() loop ---
    let words = vec!["alpha", "beta", "gamma"];
    let mut it = words.iter();
    while let Some(word) = it.next() {
        println!("Word: {word}");
    }

    // --- 4. Consuming adapters (call next() internally) ---
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];

    // .count() — counts remaining elements
    let count = nums.iter().count();
    println!("Count: {count}");

    // .sum() — sums all elements (requires std::ops::Add)
    let sum: i32 = nums.iter().sum();
    println!("Sum: {sum}");

    // .product() — multiplies all elements
    let product: i32 = nums.iter().product();
    println!("Product: {product}");

    // .last() — returns the last element
    let last = nums.iter().last();
    println!("Last: {:?}", last);

    // .nth(n) — returns the nth element (0-indexed), skipping previous
    let third = nums.iter().nth(3);
    println!("3rd element: {:?}", third);

    // --- 5. Iterator from a Range (std::ops::Range) ---
    let range_sum: i32 = (1..=10).sum();
    println!("Sum of 1..=10: {range_sum}");

    // --- 6. Iterating over a slice ---
    let slice: &[i32] = &[100, 200, 300];
    for v in slice.iter() {
        println!("Slice element: {v}");
    }

    println!("\nAll Iterator trait basics demonstrated.");
}
