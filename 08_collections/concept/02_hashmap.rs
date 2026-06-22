fn main() {
    // HashMap<K, V> — key-value store. Like C's hashmap library but built-in.
    // Use HashMap when you need to look up values by a key (not by index).
    use std::collections::HashMap;

    // --- Create ---
    let mut scores: HashMap<String, i32> = HashMap::new();

    // --- Insert ---
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 82);
    scores.insert(String::from("Charlie"), 91);

    // --- Get (returns Option<&V>) ---
    match scores.get("Alice") {
        Some(score) => println!("Alice: {}", score),
        None        => println!("Alice not found"),
    }

    // --- Entry API — insert if missing, update if present ---
    // entry() returns an Entry enum — one of the most powerful HashMap features
    scores.entry(String::from("Bob")).or_insert(0);   // Bob exists, no change
    scores.entry(String::from("Diana")).or_insert(88); // Diana missing, insert

    println!("Scores: {:?}", scores);

    // --- Iterate ---
    for (name, score) in &scores {
        println!("{} => {}", name, score);
    }

    // --- Remove ---
    let old = scores.remove("Charlie");  // returns Option<V>
    println!("Removed Charlie: {:?}", old);

    // --- Contains key ---
    println!("Has Alice? {}", scores.contains_key("Alice"));
    println!("Has Charlie? {}", scores.contains_key("Charlie"));
}
