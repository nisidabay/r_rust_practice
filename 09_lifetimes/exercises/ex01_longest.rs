// ex01_longest.rs — Find the longest string in a slice, return the longest reference.
//
// TODO: Implement `find_longest` that returns a reference to the longest string
// in the slice. If there are ties, return the first one. If empty, return None.
//
// The function takes &'a [&'a str] — a slice of string references. The lifetime
// annotation 'a links the input slice, the inner strings, and the return value.

/// Returns a reference to the longest string in the slice.
/// If multiple strings share the maximum length, returns the first one.
/// If the slice is empty, returns None.
fn find_longest<'a>(strings: &'a [&'a str]) -> Option<&'a str> {
    todo!("return a reference to the longest string")
}

/// Bonus: returns the longest of two string slices.
/// This one DOES need an explicit lifetime annotation — can you see why?
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

    println!("\n✓ ex01_longest passed!");
}
