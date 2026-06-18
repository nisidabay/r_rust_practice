// 06_lifetime_subtyping.rs
// Lifetime subtyping — relationships between lifetimes: 'a: 'b meaning "'a outlives 'b"
//
// In Rust, lifetimes form a partial order. One lifetime can "outlive" another.
// The syntax 'a: 'b means "'a is at least as long as 'b" ('a outlives or equals 'b).
//
// This matters when you want to:
//   1. Return a reference that may come from a LONER-lived input
//   2. Store references with different lifetimes in the same struct
//   3. Coerce a longer lifetime to a shorter one (which is always safe)
//
// Subtyping rule: 'long can always be used where 'short is expected.
//
// Run: rustc --edition 2021 06_lifetime_subtyping.rs && ./06_lifetime_subtyping

use std::fmt::Debug;

/// Demonstrates the simplest subtyping: &'long can become &'short.
/// The parameter wants &'a str, but we can pass &'long str.
fn examine<'a>(s: &'a str) -> &'a str {
    println!("Examining: '{s}'");
    s
}

/// Using 'a: 'b to show that 'a outlives 'b.
/// This function takes two refs where the first MUST outlive the second.
/// It returns the first one (which is guaranteed valid as long as we need).
fn first_outlives_second<'a, 'b: 'a>(first: &'a str, _second: &'b str) -> &'a str {
    first
}

/// A more practical example: store a reference with a longer lifetime
/// alongside one with a shorter lifetime, showing that the shorter one
/// can be coerced to match.
#[derive(Debug)]
struct Pair<'short, 'long: 'short> {
    /// A reference that lives at most 'short
    short_ref: &'short str,
    /// A reference that lives at least 'long, and 'long outlives 'short
    long_ref: &'long str,
}

impl<'short, 'long: 'short> Pair<'short, 'long> {
    fn new(short: &'short str, long: &'long str) -> Self {
        Pair {
            short_ref: short,
            long_ref: long,
        }
    }

    /// Both references are valid for 'short (the shorter one).
    fn display(&self) {
        println!("  short: '{}'", self.short_ref);
        println!("  long:  '{}'", self.long_ref);
    }
}

/// Variance: &'a T is covariant in 'a.
/// This means &'long T is a subtype of &'short T when 'long: 'short.
/// In practice: you can pass &'long str where &'short str is expected.
fn accept_short<'short>(s: &'short str) -> &'short str {
    s
}

/// A struct that has a method requiring 'long to outlive 'short.
/// This is the most common way you see lifetime subtyping.
#[derive(Debug)]
struct Document<'a> {
    title: &'a str,
    body: &'a str,
}

impl<'a> Document<'a> {
    /// The input reference 'b must be usable within 'a's scope.
    /// Here, we compare the document's body with an external reference.
    fn contains_word<'b>(&self, word: &'b str) -> bool
    where
        'a: 'b, // document must live at least as long as the word ref
    {
        self.body.contains(word)
    }

    /// Merge with another document — returns an owned String to avoid
    /// lifetime issues with temporary values.
    fn merge<'b>(&self, other: &'b Document<'b>) -> String
    where
        'a: 'b, // self's references must outlive other's
    {
        format!("{}\n\n{}", self.body, other.body)
    }
}

fn main() {
    // --- Basic subtyping: long -> short ---
    let long_lived = String::from("I live in the outer scope");
    {
        let short_lived = String::from("I'm temporary");
        // examine() takes &'a str — both work fine
        let res1 = examine(&long_lived);  // &'long coerced to &'short (inside this block)
        let res2 = examine(&short_lived);
        println!("Results: '{res1}', '{res2}'");
    }
    // long_lived still alive here

    // --- first_outlives_second ---
    let outer = String::from("outer scope data");
    {
        let inner = String::from("inner scope data");
        let result = first_outlives_second(&outer, &inner);
        println!("first_outlives_second: '{result}'");
        // result borrows from outer — valid ✓
    }

    // --- Pair with subtyping ---
    let permanent = String::from("permanent data");
    {
        let temp = String::from("temporary data");
        let pair = Pair::new(&temp, &permanent);
        println!("Pair:");
        pair.display();
        // pair lives only as long as temp ✓
    }

    // --- accept_short with coercion ---
    let data = String::from("some data");
    let result = accept_short(&data);
    // &data is &'long (data lives to end of main)
    // accept_short accepts &'short (any shorter lifetime)
    // Rust automatically coerces &'long to &'short ✓
    println!("accept_short result: '{result}'");

    // --- Document with lifetime bounds ---
    let doc = Document {
        title: "My Document",
        body: "This is the body of the document with important information",
    };

    // contains_word with 'a: 'b constraint
    {
        let word = "important";
        if doc.contains_word(word) {
            println!("Document contains '{word}'");
        }
    }

    // merge with lifetime bounds
    let other_doc = Document {
        title: "Other",
        body: "Additional content",
    };
    let merged = doc.merge(&other_doc);
    println!("Merged: '{}'", merged);

    // --- Practical example: iterating over data with lifetimes ---
    let all_data = vec!["apple", "banana", "cherry", "date"];
    let prefix = "b";
    {
        // filtered holds references into all_data
        let filtered: Vec<&str> = all_data
            .iter()
            .filter(|s| s.starts_with(prefix))
            .copied()
            .collect();
        println!("\nFiltered (starts with '{prefix}'): {:?}", filtered);
    }

    println!("\n✓ Lifetime subtyping understood!");
}
