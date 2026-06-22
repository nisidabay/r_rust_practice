// 01_elision.rs — Lifetime elision: the compiler fills in lifetimes for you
//
// Rust has three rules that let you omit explicit lifetime annotations
// in common patterns. The compiler adds them automatically.

// Rule 1: each input reference gets its own lifetime parameter.
// Rule 2: if there's exactly one input lifetime, assign it to all output references.
// Rule 3: if &self or &mut self, assign self's lifetime to all outputs.
//
// fn first_word(s: &str) -> &str
//   → compiler desugars to: fn first_word<'a>(s: &'a str) -> &'a str

// Rule 2 applied: one input, one output — compiler maps input lifetime to output.
fn first_word(s: &str) -> &str {
    // split_whitespace returns an iterator over &str slices borrowed from s
    s.split_whitespace().next().unwrap_or("")
}

// Rule 1 applied: two inputs, each gets its own lifetime.
// Output has no reference, so no lifetime needed.
fn compare_len(x: &str, y: &str) -> bool {
    x.len() > y.len()
}

// Rule 3 applied: &self means method output borrows from self.
struct Greeter<'a> {
    name: &'a str,
}

impl<'a> Greeter<'a> {
    // greet(&self) -> &str — self's lifetime flows to output
    fn greet(&self) -> &str {
        self.name
    }
}

fn main() {
    // Demonstrate elision in action
    let sentence = "hello world rust";
    let w = first_word(sentence);
    println!("first_word: '{}' (borrows from '{}')", w, sentence);

    println!("compare 'hello' and 'world': {}", compare_len("hello", "world"));

    let g = Greeter { name: "Alice" };
    println!("Greeter says: {}", g.greet());

    // These all compile because the compiler inferred the lifetimes.
    // No manual annotations needed.
    println!("Elision rules: Rust filled in the lifetimes automatically!");
}
