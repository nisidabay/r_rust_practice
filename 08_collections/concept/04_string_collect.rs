fn main() {
    // Building strings from collections — common patterns
    // Use collect() on iterators, join() on slices, or fold() for custom work

    // --- Join — concatenate a slice of strings with a separator ---
    let words = vec!["this", "is", "a", "sentence"];
    let sentence = words.join(" ");  // separator between elements
    println!("join: '{}'", sentence);

    // join also works on Vec<String> and &[&str]
    let csv = vec!["apple", "banana", "cherry"].join(",");
    println!("csv: '{}'", csv);

    // --- Collect from iterator ---
    // Collect any iterator of &str or String into a single String
    let chars = ['H', 'e', 'l', 'l', 'o'];
    let hello: String = chars.iter().collect();   // from char iterator
    println!("collect chars: '{}'", hello);

    // Map numbers to strings, then collect
    let nums_str: String = (1..=5)
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("nums: '{}'", nums_str);

    // --- Concat — simpler than join when there's no separator ---
    // Equivalent to join("")
    let pieces = vec!["a", "b", "c"];
    let combined = pieces.concat();
    println!("concat: '{}'", combined);

    // --- Format! — most flexible, like sprintf ---
    let desc = format!("{} items: [{}]", words.len(), words.join(", "));
    println!("format!: {}", desc);

    // --- Practical: join a filtered list ---
    let numbers = [1, 2, 3, 4, 5, 6];
    let evens: String = numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("evens: {}", evens);  // "2, 4, 6"
}
