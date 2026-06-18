// ex01_filter_map — Use filter+map chain to process data pipeline
//
// Given a list of strings like ["apple", "42", "banana", "33", "7", "cherry"],
// parse each string as an i32, filter out failures, double each successful
// number, and sum the result.
//
// Run with: ./ex01_filter_map

use std::env;

fn main() {
    // Use CLI args if provided, otherwise use defaults
    let args: Vec<String> = env::args().collect();
    let inputs: Vec<String> = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        vec![
            "apple".to_string(),
            "42".to_string(),
            "banana".to_string(),
            "33".to_string(),
            "7".to_string(),
            "cherry".to_string(),
        ]
    };

    // TODO: Use filter_map to parse strings as i32, double each, then sum
    // filter_map combines filter and map: it returns Option<T>,
    // skipping None results and unwrapping Some values.
    let sum: i32 = inputs
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .map(|n| n * 2)
        .sum();

    println!("Inputs: {:?}", inputs);
    println!("Sum of doubled parsed values: {sum}");

    // Verify
    // Parsed: 42, 33, 7 → doubled: 84, 66, 14 → sum = 164
    assert_eq!(sum, 164, "Expected sum of doubled parsed values to be 164");
    println!("Correct! The pipeline works.");
}
