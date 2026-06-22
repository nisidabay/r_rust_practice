// ex01_longest.rs — Write a function that returns the longer of two strings
//
// Implement fn longest that takes two &str and returns the longer one.
// Test with different string types (String, &str, String slices).
// Use explicit lifetime annotations.

// TODO: Write fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
// that returns the longer string (by character count, not byte length)
// HINT: use .chars().count() instead of .len() for Unicode safety
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.chars().count() >= y.chars().count() { x } else { y }
}

fn main() {
    // Test 1: two string literals (&'static str)
    let r1 = longest("hello", "world!");
    println!("Test 1 (literals): longest is '{}'", r1);

    // Test 2: one String, one &str
    let s = String::from("longer string here");
    let r2 = longest(&s, "short");
    println!("Test 2 (String vs literal): longest is '{}'", r2);

    // Test 3: both Strings
    let a = String::from("aaaa");
    let b = String::from("bbbbbb");
    let r3 = longest(&a, &b);
    println!("Test 3 (two Strings): longest is '{}'", r3);

    // Test 4: Unicode — "café" has 4 chars, "coffee" has 6
    let r4 = longest("café", "coffee");
    println!("Test 4 (unicode): longest is '{}'", r4);

    // Test 5: equal length — should return x (due to >=)
    let r5 = longest("same", "same");
    println!("Test 5 (equal): longest is '{}'", r5);

    println!("\nAll tests passed! ✓");
}
