// 06_cow.rs — Cow (Clone-on-Write), borrow or own pattern
//
// Cow<B: ToOwned + ?Sized> is an enum that either Borrowed(&T) or Owned(T).
// It lets functions return borrowed data when possible, and owned data
// only when mutation is needed. This avoids unnecessary clones.
//
// Cow is especially useful for:
// 1. Functions that sometimes need to modify input but mostly don't
// 2. Returning either a string literal or a computed String
// 3. APIs where you want to defer allocation until mutation is required

use std::borrow::Cow;

fn main() {
    // Cow::Borrowed wraps a reference — no allocation
    let borrowed: Cow<str> = Cow::Borrowed("hello");

    // Cow::Owned wraps an owned value
    let owned: Cow<str> = Cow::Owned(String::from("world"));

    // to_mut() converts Borrowed -> Owned only when mutation is needed.
    // If already Owned, it just returns &mut without clone.
    let mut cow: Cow<str> = Cow::Borrowed("hello");
    assert!(matches!(cow, Cow::Borrowed(_)));

    // This triggers a clone — the borrowed data is copied to an owned String
    // so we can mutate it. After this, cow is Cow::Owned.
    let s = cow.to_mut();
    s.push_str(" world");
    assert!(matches!(cow, Cow::Owned(_)));
    println!("After to_mut: {}", cow);

    // If cow is already Owned, to_mut() just returns &mut — no allocation
    let mut cow2: Cow<str> = Cow::Owned(String::from("hello"));
    let s = cow2.to_mut();
    s.push_str(" universe");
    println!("Already owned, mutated in place: {}", cow2);

    // Practical: case-insensitive prefix check with optional normalization
    let tests = vec!["Hello World", "ROME", "Paris"];
    for test in &tests {
        let normalized = normalize_case(test);
        println!("'{}' -> '{}'", test, normalized);
    }

    // Practical: title-casing with Cow avoids allocation for already-correct strings
    let samples = vec!["hello world", "Hello World", "rust programming"];
    for sample in &samples {
        let titled = title_case(sample);
        println!("'{}' -> '{}' (borrowed: {})", sample, titled, matches!(titled, Cow::Borrowed(_)));
    }

    // Cow also works with other types: [T], Path, OsStr, etc.
    demonstrate_cow_vec();
}

/// Normalizes a name: lowercase if it's all-uppercase, otherwise return as-is.
/// Uses Cow to avoid allocation in the common case (already lowercase or mixed case).
fn normalize_case(name: &str) -> Cow<str> {
    if name.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
        // Only uppercase — allocate and lowercase
        Cow::Owned(name.to_lowercase())
    } else {
        // Already normal — return borrowed, no allocation
        Cow::Borrowed(name)
    }
}

/// Title-cases a string, but only allocates if the string isn't already
/// properly title-cased.
fn title_case(s: &str) -> Cow<str> {
    let mut result = String::with_capacity(s.len());
    let mut changed = false;
    let mut new_word = true;

    for c in s.chars() {
        let next = if new_word {
            new_word = false;
            c.to_uppercase().next().unwrap()
        } else {
            c.to_lowercase().next().unwrap()
        };
        if c == ' ' {
            new_word = true;
        }
        if next != c {
            changed = true;
        }
        result.push(next);
    }

    if changed {
        Cow::Owned(result)
    } else {
        Cow::Borrowed(s) // no allocation needed
    }
}

/// Demonstrates Cow with Vec<T> and slices
fn demonstrate_cow_vec() {
    // Cow<[i32]> — either borrowed &[i32] or owned Vec<i32>
    let data: Cow<[i32]> = Cow::Borrowed(&[1, 2, 3, 4, 5]);

    // to_mut converts to owned Vec only when we modify
    let mut data = data;
    let slice = data.to_mut();
    slice.push(6);
    println!("Cow<[i32]> after push: {:?}", data);

    // without mutation, Cow<[i32]> stays borrowed
    let static_data: Cow<[i32]> = Cow::Borrowed(&[10, 20, 30]);
    println!(
        "Cow<[i32]> without mutation (borrowed? {}): {:?}",
        matches!(static_data, Cow::Borrowed(_)),
        static_data
    );
}
