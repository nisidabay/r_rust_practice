// 08_iterator_composition — Chaining multiple adapters for complex pipelines
//
// The real power of iterators comes from composing multiple adapters
// into a single declarative pipeline. Each adapter transforms the stream,
// and the pipeline only executes when collected or consumed.

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // --- 1. Basic pipeline: filter → map → take ---
    // Take the first 3 even numbers and square them
    let result: Vec<i32> = data
        .iter()
        .filter(|x| *x % 2 == 0)    // keep evens
        .map(|x| x * x)              // square them
        .take(3)                     // first 3
        .collect();
    println!("First 3 even squares: {:?}", result);

    // --- 2. Filter → map → sum (no collection needed) ---
    let sum_of_squares: i32 = data
        .iter()
        .filter(|x| *x % 2 != 0)     // odds only
        .map(|x| x * x)              // square
        .sum();                      // sum them up
    println!("Sum of squares of odds: {sum_of_squares}");

    // --- 3. Practical: word filtering pipeline ---
    let text = vec![
        "hello", "world", "rust", "is", "awesome", "a",
        "functional", "iterator", "pipeline", "hi",
    ];

    // Find words longer than 2 chars, uppercase them, take first 5
    let processed: Vec<String> = text
        .iter()
        .filter(|w| w.len() > 2)        // skip short words
        .map(|w| w.to_uppercase())       // uppercase
        .take(5)                         // first 5
        .collect();
    println!("Processed words: {:?}", processed);

    // --- 4. Chained with enumerate and skip ---
    let nums = vec![10, 20, 30, 40, 50, 60, 70];

    // Skip first 2, take 3, add index, format as string
    let formatted: Vec<String> = nums
        .iter()
        .skip(2)
        .take(3)
        .enumerate()
        .map(|(i, val)| format!("#{}: {}", i + 1, val))
        .collect();
    println!("Formatted: {:?}", formatted);

    // --- 5. Flatten nested structures ---
    let nested = vec![
        vec![1, 2, 3],
        vec![4, 5],
        vec![6, 7, 8, 9],
    ];

    // Flatten and filter evens
    let flat_evens: Vec<i32> = nested
        .iter()
        .flat_map(|v| v.iter())          // flatten each inner vec
        .filter(|x| *x % 2 == 0)
        .cloned()
        .collect();
    println!("Flat evens: {:?}", flat_evens);

    // --- 6. filter_map — filter and map in one step ---
    let strs = vec!["1", "two", "3", "four", "5", "six"];

    // Try to parse each string as i32, keep only successful parses
    let parsed: Vec<i32> = strs
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Parsed numbers: {:?}", parsed);

    // --- 7. Complex pipeline: scoring ---
    struct Player {
        name: &'static str,
        score: i32,
    }

    let players = vec![
        Player { name: "Alice", score: 95 },
        Player { name: "Bob", score: 42 },
        Player { name: "Carol", score: 78 },
        Player { name: "Dave", score: 61 },
    ];

    // Top 2 players with score >= 70, sorted descending
    let mut top_players: Vec<&Player> = players.iter().collect();
    top_players.sort_by(|a, b| b.score.cmp(&a.score));

    let top2: Vec<&str> = top_players
        .iter()
        .filter(|p| p.score >= 70)
        .take(2)
        .map(|p| p.name)
        .collect();
    println!("Top 2 players (score >= 70): {:?}", top2);

    println!("\nAll iterator composition patterns demonstrated.");
}
