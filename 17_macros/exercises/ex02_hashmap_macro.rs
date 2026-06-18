// ex02_hashmap_macro.rs — Build a hashmap! macro like vec!
//
// Exercise: Create a `hashmap!` macro that constructs a HashMap from
// key-value pairs with ergonomic syntax.

use std::collections::HashMap;

/// Build a HashMap from key-value pairs.
/// Syntax: hashmap! { key => value, key2 => value2, ... }
/// Supports trailing commas and empty map.
macro_rules! hashmap {
    // Empty map
    () => {
        HashMap::new()
    };

    // One or more key => value pairs, with optional trailing comma
    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut map = HashMap::new();
        $(
            map.insert($key, $value);
        )+
        map
    }};

    // Alternative: comma-separated pairs with (key, value) syntax
    // Not included to avoid ambiguity — use => for clarity.
}

/// A version that pre-allocates capacity for performance.
macro_rules! hashmap_with_capacity {
    ($($key:expr => $value:expr),+ $(,)?) => {{
        let count = hashmap_count!($($key),+);
        let mut map = HashMap::with_capacity(count);
        $(
            map.insert($key, $value);
        )+
        map
    }};
}

/// Helper: count keys to pre-allocate.
macro_rules! hashmap_count {
    () => { 0usize };
    ($first:expr $(, $rest:expr)*) => { 1usize };
    ($first:expr, $($rest:expr),+) => {
        1usize + hashmap_count!($($rest),+)
    };
}

fn main() {
    println!("=== hashmap! Macro ===");

    // Basic usage
    let scores = hashmap! {
        "Alice" => 95,
        "Bob" => 87,
        "Charlie" => 92,
    };
    println!("Scores: {:?}", scores);

    // Trailing comma
    let config = hashmap! {
        "host" => "localhost",
        "port" => "8080",
    };
    println!("Config: {:?}", config);

    // Empty
    let empty: HashMap<&str, i32> = hashmap! {};
    println!("Empty map: {:?}", empty);

    // With capacity pre-allocation
    let large: HashMap<&str, u32> = hashmap_with_capacity! {
        "a" => 1,
        "b" => 2,
        "c" => 3,
    };
    println!("With capacity: {:?}", large);

    // Mixed types (value types must be consistent)
    let lookup = hashmap! {
        1 => "one",
        2 => "two",
        3 => "three",
    };
    println!("Integer keys: {:?}", lookup);

    println!("\nKey features:");
    println!("- `key => value` syntax (similar to Ruby/map literals)");
    println!("- Trailing comma support");
    println!("- Empty map support");
    println!("- Full type inference (no turbofish needed)");
}
