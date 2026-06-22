// ex03_joiner.rs — Join string slices with a separator
//
// Write fn join_with_sep<'a>(parts: &[&'a str], sep: &'a str) -> String
// that builds a String by joining all parts with the separator.
//
// This is like "a".join(&["b", "c", "d"]) in Python = "b c d"
// or String::join in Rust, but we implement it manually.

// TODO: Implement join_with_sep that concatenates parts with sep between them.
// Use a loop, push_str, and avoid trailing separator.
fn join_with_sep<'a>(parts: &[&'a str], sep: &'a str) -> String {
    let mut result = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            result.push_str(sep);
        }
        result.push_str(part);
    }
    result
}

fn main() {
    // Test 1: basic join
    let words = ["hello", "world", "rust"];
    let joined = join_with_sep(&words, " ");
    println!("Test 1 (space): '{}'", joined);
    assert_eq!(joined, "hello world rust");

    // Test 2: comma separator
    let items = ["apple", "banana", "cherry"];
    let csv = join_with_sep(&items, ", ");
    println!("Test 2 (comma): '{}'", csv);
    assert_eq!(csv, "apple, banana, cherry");

    // Test 3: single element
    let single = ["only"];
    let s = join_with_sep(&single, ", ");
    println!("Test 3 (single): '{}'", s);
    assert_eq!(s, "only");

    // Test 4: empty slice
    let empty: &[&str] = &[];
    let e = join_with_sep(empty, ", ");
    println!("Test 4 (empty): '{}'", e);
    assert_eq!(e, "");

    // Test 5: different separator types
    let nums = ["42", "7", "99"];
    let pipe = join_with_sep(&nums, "|");
    println!("Test 5 (pipe): '{}'", pipe);
    assert_eq!(pipe, "42|7|99");

    println!("\nAll tests passed! ✓");
}
