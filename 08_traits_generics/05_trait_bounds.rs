// 05_trait_bounds.rs
// Where clauses and trait bounds — T: Trait1 + Trait2.
//
// Trait bounds constrain generic type parameters. They tell the compiler
// "this function only works with types that implement these traits."
// The `where` clause is the clearest syntax for complex bounds.
//
// Run: rustc --edition 2021 05_trait_bounds.rs && ./05_trait_bounds

use std::cmp::PartialOrd;
use std::fmt::Display;

// ── Inline bounds ────────────────────────────────────────────────────────────

/// Classic bound written inline: T must implement Display and PartialOrd.
fn describe_biggest<T: Display + PartialOrd>(a: T, b: T) {
    if a > b {
        println!("Biggest: {a}");
    } else {
        println!("Biggest: {b}");
    }
}

// ── Where clause (cleaner for complex bounds) ────────────────────────────────

/// Returns the median of three values. The `where` clause is preferred when
/// bounds are long or there are multiple type parameters.
fn median_of_three<T>(a: T, b: T, c: T) -> T
where
    T: PartialOrd + Copy,
{
    let mut v = [a, b, c];
    if v[0] > v[1] {
        v.swap(0, 1);
    }
    if v[1] > v[2] {
        v.swap(1, 2);
    }
    if v[0] > v[1] {
        v.swap(0, 1);
    }
    v[1]
}

/// Better version with Clone bound so it works on non-Copy types too.
fn median_of_three_clone<T>(a: T, b: T, c: T) -> T
where
    T: PartialOrd + Clone,
{
    let mut v = [a, b, c];
    if v[0] > v[1] {
        v.swap(0, 1);
    }
    if v[1] > v[2] {
        v.swap(1, 2);
    }
    if v[0] > v[1] {
        v.swap(0, 1);
    }
    v[1].clone()
}

// ── Multiple type parameters with different bounds ──────────────────────────

/// Zips two slices into a vector of pairs, showing how different type params
/// can have different bounds.
fn zip_display<T, U>(a: &[T], b: &[U]) -> Vec<(String, String)>
where
    T: Display,
    U: Display,
{
    let min_len = a.len().min(b.len());
    (0..min_len)
        .map(|i| (format!("{}", a[i]), format!("{}", b[i])))
        .collect()
}

// ── Associated trait bound syntax (T: Trait<Associated=...>) ─────────────────

use std::ops::Add;

/// Sums all items in a slice using Add. T must support adding T to produce T.
fn sum_all<T>(items: &[T]) -> Option<T>
where
    T: Add<Output = T> + Clone + Default,
{
    if items.is_empty() {
        return None;
    }
    let mut total = T::default();
    for item in items {
        total = total + item.clone();
    }
    Some(total)
}

// ── Higher-ranked trait bounds (HRTB) for closures ───────────────────────────

/// Applies a function to a reference and returns the result.
/// The `for<'a>` syntax says "this function must work for ANY lifetime".
fn apply_to_ref<T, R>(x: &T, f: impl for<'a> Fn(&'a T) -> R) -> R {
    f(x)
}

fn main() {
    describe_biggest(10, 20);
    describe_biggest(3.14, 2.71);

    // median_of_three requires Copy (because we move out of array).
    let m = median_of_three(100, 50, 75);
    println!("median = {m}");

    // median_of_three_clone works with Strings (Clone but not Copy).
    let m2 = median_of_three_clone("apple".to_string(), "cherry".to_string(), "banana".to_string());
    println!("median string = {m2}");

    // zip display
    let nums = [1, 2, 3];
    let names = ["one", "two", "three", "four"];
    let zipped = zip_display(&nums, &names);
    for (n, name) in zipped {
        println!("  {n} -> {name}");
    }

    // sum with Add bound
    let values = vec![1.0, 2.0, 3.0, 4.0];
    match sum_all(&values) {
        Some(s) => println!("sum = {s}"),
        None => println!("empty sum"),
    }

    // HRTB with closure
    let s = String::from("hello");
    let len = apply_to_ref(&s, |x: &String| x.len());
    println!("apply_to_ref length = {len}");
}
