// 07_lifetime_impl.rs
// Lifetime annotations in impl blocks and methods — tying it all together.
//
// When you have a struct with lifetime parameters, EVERY impl block must
// declare those lifetimes. The syntax `impl<'a> Struct<'a>` makes 'a available
// to all methods inside the block.
//
// Key patterns:
//   1. impl<'a> — one lifetime for the struct
//   2. impl<'a, 'b> — multiple lifetimes for the struct
//   3. Methods can introduce NEW lifetimes in their signatures
//   4. Associated functions (no &self) need explicit lifetime annotations
//
// Run: rustc --edition 2021 07_lifetime_impl.rs && ./07_lifetime_impl

use std::fmt::Display;

// ─── Example 1: Simple struct, one lifetime ─────────────────────────────────

struct Wrapper<'a> {
    data: &'a str,
}

// Each impl block declares <'a> to say "this block works for any 'a"
impl<'a> Wrapper<'a> {
    /// Constructor — returns Self with the same lifetime
    fn new(data: &'a str) -> Self {
        Wrapper { data }
    }

    /// Getter — output lifetime elided to 'a via &self rule
    fn get(&self) -> &str {
        self.data
    }

    /// Method with its OWN lifetime parameter (not on the struct)
    fn contains<'b>(&self, other: &'b str) -> bool
    where
        'a: 'b,
    {
        self.data.contains(other)
    }

    /// A static method — no &self, must be explicit about lifetimes
    fn create_pair(x: &'a str, y: &'a str) -> (Self, Self) {
        (Wrapper { data: x }, Wrapper { data: y })
    }
}

// ─── Example 2: Two lifetimes, two impl blocks ─────────────────────────────

struct MultiRef<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

// impl block for when both lifetimes are the same
impl<'a> MultiRef<'a, 'a> {
    fn both_same(first: &'a str, second: &'a str) -> Self {
        MultiRef { first, second }
    }

    fn display(&self) {
        println!("  first: '{}' (same lifetime)", self.first);
        println!("  second: '{}' (same lifetime)", self.second);
    }
}

// impl block for when lifetimes are DIFFERENT
impl<'a, 'b> MultiRef<'a, 'b> {
    fn different(first: &'a str, second: &'b str) -> Self {
        MultiRef { first, second }
    }

    fn first(&self) -> &'a str {
        self.first
    }

    fn second(&self) -> &'b str {
        self.second
    }
}

// ─── Example 3: Generic struct with lifetime + type parameter ───────────────

struct BorrowedValue<'a, T: ?Sized> {
    value: &'a T,
}

impl<'a, T: ?Sized> BorrowedValue<'a, T> {
    fn new(value: &'a T) -> Self {
        BorrowedValue { value }
    }

    fn get(&self) -> &'a T {
        self.value
    }
}

// Specific impl for BorrowedValue<str>
impl<'a> BorrowedValue<'a, str> {
    fn len(&self) -> usize {
        self.value.len()
    }

    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

// ─── Example 4: Method that introduces a NEW unrelated lifetime ─────────────

struct Data<'d> {
    content: &'d str,
}

impl<'d> Data<'d> {
    /// Returns true if content contains the given word.
    /// 'w is a NEW lifetime (not connected to 'd)
    fn has_word<'w>(&self, word: &'w str) -> bool {
        self.content.contains(word)
    }

    /// Combines this Data with another — 'o is a new lifetime.
    /// Returns a String (owned), so no lifetime annotations on output.
    fn combine<'o>(&self, other: &'o Data<'o>) -> String {
        format!("{}{}", self.content, other.content)
    }
}

// ─── Example 5: Lifetimes on trait impl ─────────────────────────────────────

trait Describable {
    fn describe(&self) -> String;
}

impl<'a> Describable for Wrapper<'a> {
    fn describe(&self) -> String {
        format!("Wrapper holds: '{}'", self.data)
    }
}

// ─── Example 6: impl with where clause lifetimes ───────────────────────────

struct Exact<'a, 'b> {
    a: &'a str,
    b: &'b str,
}

// Only available when 'a: 'b (a outlives b)
impl<'a, 'b> Exact<'a, 'b>
where
    'a: 'b,
{
    fn new(a: &'a str, b: &'b str) -> Self {
        Exact { a, b }
    }

    /// Returns b (which is valid for 'b) — but 'a is longer, so we can
    /// still return &'a str since 'a: 'b allows coercing &'b to &'a... 
    /// Actually no — &'b is SHORTER, you can't return it as &'a without 
    /// a constraint the other direction. Let's just return &str (elided).
    fn get_b(&self) -> &str {
        self.b
    }
}

fn main() {
    // --- Wrapper ---
    let text = String::from("Rust lifetimes are powerful");
    let wrapper = Wrapper::new(&text);
    println!("Wrapper.get(): '{}'", wrapper.get());
    println!("Wrapper.contains('powerful'): {}", wrapper.contains("powerful"));

    let (w1, w2) = Wrapper::create_pair("first", "second");
    println!("Pair: '{}', '{}'", w1.get(), w2.get());

    // --- MultiRef ---
    let same = MultiRef::both_same("hello", "world");
    same.display();

    let s1 = String::from("longer lived");
    let s2 = String::from("shorter");
    let different = MultiRef::different(&s1, &s2);
    println!(
        "MultiRef.different(): first='{}', second='{}'",
        different.first(),
        different.second()
    );

    // --- BorrowedValue ---
    let num = 100u64;
    let bv = BorrowedValue::new(&num);
    println!("BorrowedValue<u64>: {}", bv.get());

    let greeting = "hello";
    let bv_str = BorrowedValue::new(greeting);
    println!("BorrowedValue<str>: '{}', len={}, empty={}", bv_str.get(), bv_str.len(), bv_str.is_empty());

    // --- Data with new lifetimes in methods ---
    let d1 = Data { content: "Apples are tasty" };
    let d2 = Data { content: " and oranges too" };
    println!("Data.has_word('tasty'): {}", d1.has_word("tasty"));
    println!("Data.combine(): {}", d1.combine(&d2));

    // --- Trait impl with lifetimes ---
    let w = Wrapper::new("describable content");
    println!("Describable: {}", w.describe());

    // --- Exact with where clause ---
    let long = String::from("long text");
    {
        let short = String::from("short");
        let exact = Exact::new(&long, &short);
        println!("Exact.get_b(): '{}'", exact.get_b());
    }

    println!("\n✓ Lifetime impl blocks understood!");
}
