fn main() {
    // HashSet<T> — a set of unique values. Like C's hash set.
    // Use HashSet when you need: "is this value in the collection?"
    use std::collections::HashSet;

    // --- Create and Insert ---
    let mut set = HashSet::new();
    set.insert("apple");
    set.insert("banana");
    set.insert("cherry");
    set.insert("apple");  // duplicate — silently ignored
    println!("Set: {:?}", set);  // only 3 elements

    // --- Contains (the main reason to use a set) ---
    println!("Has banana? {}", set.contains("banana"));  // true
    println!("Has durian? {}", set.contains("durian"));  // false

    // --- Union, Intersection, Difference ---
    // These return iterators, not new HashSets (zero-copy)
    let a: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let b: HashSet<i32> = [4, 5, 6, 7, 8].iter().cloned().collect();

    // Union — elements in a OR b
    let union: Vec<_> = a.union(&b).collect();
    println!("Union: {:?}", union);  // [1, 2, 3, 4, 5, 6, 7, 8]

    // Intersection — elements in a AND b
    let intersection: Vec<_> = a.intersection(&b).collect();
    println!("Intersection: {:?}", intersection);  // [4, 5]

    // Difference — elements in a but NOT b
    let diff: Vec<_> = a.difference(&b).collect();
    println!("Difference (a - b): {:?}", diff);  // [1, 2, 3]

    // Symmetric difference — elements in a or b but not both
    let sym_diff: Vec<_> = a.symmetric_difference(&b).collect();
    println!("Symmetric diff: {:?}", sym_diff);  // [1, 2, 3, 6, 7, 8]
}
