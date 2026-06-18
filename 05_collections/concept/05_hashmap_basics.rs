// 05_hashmap_basics.rs — HashMap, insert, get, entry, or_insert
//
// HashMap<K, V> maps keys to values with O(1) average lookup.
// First 5 lines: create, insert, get, entry API, or_insert.

use std::collections::HashMap;

fn main() {
    // 1. HashMap::new — start empty (needs type annotation or usage)
    let mut scores = HashMap::new();

    // 2. insert — add or update a key-value pair
    scores.insert(String::from("Alice"), 42);
    scores.insert(String::from("Bob"), 17);

    // 3. get — returns Option<&V>
    match scores.get("Alice") {
        Some(val) => println!("Alice scored {val}"),
        None => println!("Alice not found"),
    }

    // 4. entry API + or_insert — insert if missing, else leave alone
    scores.entry(String::from("Charlie")).or_insert(99);
    scores.entry(String::from("Alice")).or_insert(0); // Alice already 42
    println!("scores: {scores:?}");

    // 5. or_insert_with — lazy initialization
    scores
        .entry(String::from("Diana"))
        .or_insert_with(|| 50);
    println!("Diana: {:?}", scores.get("Diana"));

    // 6. remove — delete a key
    scores.remove("Bob");
    println!("after remove: {scores:?}");

    // 7. len / contains_key
    println!("{} entries, contains Alice? {}", scores.len(), scores.contains_key("Alice"));

    // 8. from iterators — collect into HashMap
    let teams = vec!["blue".to_string(), "red".to_string()];
    let points = vec![10, 20];
    let team_scores: HashMap<_, _> = teams.into_iter().zip(points.into_iter()).collect();
    println!("from zip: {team_scores:?}");
}
