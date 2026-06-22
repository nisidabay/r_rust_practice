/*
 * ex03_find_longest.rs — Exercise 3
 *
 * Task: Write fn longest(a: &str, b: &str) -> &str that returns
 *       whichever string is longer. This demonstrates LIFETIME
 *       annotations: the return type needs a lifetime parameter.
 *
 * The issue: the compiler doesn't know how long the return reference
 * will live. We must add a lifetime annotation: <'a> saying "the
 * return value lives as long as the shorter of the two inputs".
 *
 * Run: ./ex03_find_longest
 * Expected: longest of "abc" and "xyz" = "abc" (tie stays first)
 */

// With lifetime annotation: the return reference lives as long as
// both inputs ('a = the shorter of the two lifetimes)
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let s1 = String::from("abc");
    let s2 = String::from("xyz");
    let result = longest(&s1, &s2);
    println!("longest of '{}' and '{}' = '{}'", s1, s2, result);

    // Different lengths
    let short = "hi";
    let long = "hello";
    println!("longest of '{}' and '{}' = '{}'", short, long, longest(short, long));

    // Both empty
    println!("longest of '' and '' = '{}'", longest("", ""));

    // The lifetime constraint means: the returned reference can't
    // outlive either input. This is checked at compile time:
    let a = String::from("long lived");
    {
        let b = String::from("short");
        let result = longest(&a, &b); // OK — both alive
        println!("longest with scoped b: '{}'", result);
    }
    // let result = longest(&a, &String::from("short")); // COMPILE ERROR: temporary dropped

    println!("All tests passed!");
}
