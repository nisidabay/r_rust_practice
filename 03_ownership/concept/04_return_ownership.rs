// Returning ownership from functions is awkward — you have to pass values back.
// The tuple pattern lets you return a value alongside its ownership.
// But this is tedious — references solve this much more elegantly.

fn main() {
    // Without references: every function that touches heap data must give it back
    let data: String = String::from("important data");

    // Process the data — but we get ownership back
    let data: String = process_and_return(data);
    println!("1. Back in main with: {}", data);

    let data: String = process_and_return(data);
    println!("2. Processed again: {}", data);

    // The tuple pattern: return owned data alongside computed values
    let text: String = String::from("hello world");
    let (word_count, text) = count_words(text);
    // We get both the count AND ownership of text back
    println!("3. '{}' has {} words", text, word_count);

    // Multiple operations — annoying without references:
    let s: String = String::from("Rust programming");
    let (s, uppered) = to_uppercase(s);
    let (s, lowered) = to_lowercase(s);
    println!("4. Upper: {}, Lower: {}", uppered, lowered);
    // We still own s — but we had to thread it through every function
    println!("5. Original still mine: {}", s);

    // === BETTER APPROACH: use references ===
    // (coming in 05_references.rs — this is just showing why we need them)
    // Reference version: fn count_words(text: &str) -> usize
    // The function borrows the data without taking ownership
    println!("6. ✓ Ownership return patterns demonstrated");
}

// Takes ownership, does something, returns ownership
fn process_and_return(s: String) -> String {
    println!("   Processing: '{}'", s);
    s // ownership returned to caller
}

// Count words in a string, return both count and the string itself
// This is the "tuple pattern" — return everything the caller might need
fn count_words(s: String) -> (usize, String) {
    let count: usize = s.split_whitespace().count();
    (count, s) // return tuple with both the computed value and ownership
}

// Demonstrate how awkward it gets with multiple transformations
fn to_uppercase(s: String) -> (String, String) {
    let uppered: String = s.to_uppercase();
    (s, uppered) // return original + transformed
}

fn to_lowercase(s: String) -> (String, String) {
    let lowered: String = s.to_lowercase();
    (s, lowered)
}
