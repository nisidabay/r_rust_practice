// 07_hashmap_iterate.rs — Iterating HashMap, for (k, v) pattern
//
// HashMap gives you three iterators: keys, values, and (key, value) pairs.
// First 5 lines: for (k, v), iter(), keys(), values(), entry iteration.

use std::collections::HashMap;

fn main() {
    let mut inventory = HashMap::new();
    inventory.insert(String::from("apples"), 5);
    inventory.insert(String::from("bananas"), 12);
    inventory.insert(String::from("oranges"), 8);

    // 1. for (k, v) — iterate over key-value references
    println!("Full inventory:");
    for (item, qty) in &inventory {
        println!("  {item}: {qty}");
    }

    // 2. .keys() — iterate over keys only
    println!("\nItems:");
    for item in inventory.keys() {
        println!("  {item}");
    }

    // 3. .values() — iterate over values only
    let total: i32 = inventory.values().sum();
    println!("\nTotal items: {total}");

    // 4. .values_mut() — modify values in place
    for qty in inventory.values_mut() {
        *qty += 1; // one more of everything
    }
    println!("After restock: {inventory:?}");

    // 5. .into_iter() — consumes the HashMap
    let items: Vec<String> = inventory.into_iter().map(|(k, _v)| k).collect();
    // println!("{inventory:?}"); // ERROR: consumed
    println!("Just the keys (consumed): {items:?}");

    // 6. collect from iterator of pairs
    let pairs = vec![("x", 1), ("y", 2), ("z", 3)];
    let map: HashMap<&str, i32> = pairs.into_iter().collect();
    println!("From iterator: {map:?}");

    // 7. len / is_empty
    println!("map len: {}", map.len());
}
