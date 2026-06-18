// ex01_longest_solved.rs — Find the longest string in a slice, return the longest reference.
//
// Solved version of ex01_longest.

/// Returns a reference to the longest string in the slice.
fn find_longest<'a>(strings: &'a [&'a str]) -> Option<&'a str> {
    if strings.is_empty() {
        return None;
    }
    let mut longest = strings[0];
    for s in strings.iter().skip(1) {
        if s.len() > longest.len() {
            longest = s;
        }
    }
    Some(longest)
}

/// Returns the longest of two string slices.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    // Test 1: basic longest
    let words = ["Rust", "is", "amazing", "and", "powerful"];
    match find_longest(&words) {
        Some(w) => println!("Longest: '{w}' (len={})", w.len()),
        None => println!("No longest found"),
    }
    assert_eq!(find_longest(&words), Some("powerful"));

    // Test 2: ties — first wins
    let ties = ["cat", "dog", "elf"];
    assert_eq!(find_longest(&ties), Some("cat"));

    // Test 3: empty slice
    let empty: [&str; 0] = [];
    assert_eq!(find_longest(&empty), None);

    // Test 4: single element
    let single = ["lonely"];
    assert_eq!(find_longest(&single), Some("lonely"));

    // Test 5: all same length — first wins
    let same = ["abc", "def", "ghi"];
    assert_eq!(find_longest(&same), Some("abc"));

    // Test 6: explicit longest
    let a = "short";
    let b = "longer_string";
    let result = longest(a, b);
    println!("Longest of '{a}' and '{b}': '{result}'");
    assert_eq!(result, b);

    println!("\n✓ ex01_longest_solved passed!");
}
