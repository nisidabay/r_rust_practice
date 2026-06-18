// 04_propagate — The ? operator, propagating errors up
//
// The ? operator is syntactic sugar for the common "return early on error"
// pattern. Instead of:
//
//   let val = match something() {
//       Ok(v) => v,
//       Err(e) => return Err(e),
//   };
//
// You write:
//
//   let val = something()?;
//
// ? can only be used in functions that return Result (or Option).

use std::fs;
use std::io;
use std::num::ParseIntError;

// --- 1. Functions that return Result ---
// ? only works if the function's return type matches.

fn read_number_from_file(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // Without ?:
    // let contents = match fs::read_to_string(path) {
    //     Ok(c) => c,
    //     Err(e) => return Err(Box::new(e)),
    // };

    // With ? — much cleaner:
    let contents = fs::read_to_string(path)?;

    // ? also works for parse:
    let num: i32 = contents.trim().parse()?;

    Ok(num)
}

// --- 2. ? converts error types automatically ---
// When the function returns a wider error type (like Box<dyn Error>),
// the ? operator auto-converts via From<E> for any concrete E.

fn main() {
    // --- 3. ? only works inside Result-returning functions ---
    // Uncommenting this inside main() (which returns ()) won't compile:
    // let x = "42".parse::<i32>()?; // ERROR: can't use ? in ()-returning fn

    // To use ? in main(), main must return Result:
    // fn main() -> Result<(), Box<dyn std::error::Error>> { ... }

    // --- 4. ? with Option ---
    // ? also works with Option — returns None early.
    fn first_item(list: &[i32]) -> Option<&i32> {
        let first = list.first()?; // returns None if list is empty
        Some(first)
    }

    println!("first_item(&[1,2,3]) = {:?}", first_item(&[1, 2, 3]));
    println!("first_item(&[]) = {:?}", first_item(&[]));

    // --- 5. Chaining with ? ---
    // ? can be used in a chain:
    fn parse_first_int(s: &str) -> Option<i32> {
        Some(s.split_whitespace().next()?.parse().ok()?)
    }

    println!("parse_first_int(\"42 foo\") = {:?}", parse_first_int("42 foo"));
    println!("parse_first_int(\"\") = {:?}", parse_first_int(""));
    println!(
        "parse_first_int(\"abc\") = {:?}",
        parse_first_int("abc")
    );

    // --- 6. The try operator in expressions ---
    // ? can also be used in let-else constructs (Rust 1.65+):
    fn get_last(v: &[i32]) -> Option<&i32> {
        let Some(last) = v.last() else { return None };
        // Now last is &i32, no ? needed
        Some(last)
    }

    println!("get_last(&[1,2,3]) = {:?}", get_last(&[1, 2, 3]));
    println!("get_last(&[]) = {:?}", get_last(&[]));

    // --- 7. Demo: read_number_from_file ---
    // This reads from a temp file; if it doesn't exist, error propagates.
    // We don't actually run it here to avoid creating temp files,
    // but the pattern is clear.

    println!("--- 04_propagate done ---");
}
