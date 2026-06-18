// 01_lifetime_elision.rs
// Lifetime elision — the three rules that let you skip writing lifetimes 90% of the time.
//
// Lifetimes are Rust's way of ensuring references are always valid. But most of the
// time, Rust can figure them out automatically. The compiler applies three elision
// rules so you don't have to write 'a everywhere.
//
// The three rules (applied in order):
//   1. Each input reference gets its OWN lifetime parameter.
//   2. If there's exactly ONE input lifetime, it's assigned to ALL output references.
//   3. If there are multiple input lifetimes but one is &self or &mut self,
//      the self lifetime is assigned to all output references.
//
// Run: rustc --edition 2021 01_lifetime_elision.rs && ./01_lifetime_elision

/// Rule 1 + 2 in action: one input reference → one output lifetime.
/// The compiler sees `&str` (input) and `&str` (output) and infers they share
/// the same lifetime. Written explicitly it would be: fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    // Find the first space, return the slice before it
    match s.find(' ') {
        Some(pos) => &s[..pos],
        None => s,
    }
}

/// Rule 2: one input reference → output gets the same lifetime.
/// No explicit lifetimes needed.
fn longest_snippet(text: &str) -> &str {
    if text.len() > 10 {
        &text[..10]
    } else {
        text
    }
}

/// Rule 2 again: one input reference → same lifetime for output.
fn trim_whitespace(s: &str) -> &str {
    s.trim()
}

/// Rule 3: &self means output gets the same lifetime as self.
/// This is extremely common in method definitions.
struct TextHolder<'a> {
    text: &'a str,
}

impl<'a> TextHolder<'a> {
    /// No explicit lifetimes needed in the method signature.
    /// Rule 3: &self gets its own lifetime, output gets the same lifetime.
    fn content(&self) -> &str {
        self.text
    }

    /// Another method where elision works.
    fn first_line(&self) -> &str {
        match self.text.find('\n') {
            Some(pos) => &self.text[..pos],
            None => self.text,
        }
    }
}

/// Rule 2 again: single input reference, multiple output references
/// All outputs share the same lifetime as the input.
fn split_at_first(s: &str, delim: char) -> Option<(&str, &str)> {
    match s.find(delim) {
        Some(pos) => Some((&s[..pos], &s[pos + 1..])),
        None => None,
    }
}

/// When elision CANNOT apply: multiple input references, no &self.
/// This function needs explicit lifetimes (we'll cover that in 02_explicit_lifetime.rs).
/// Here we cheat by taking one reference at a time.
fn choose_prefix<'a>(s: &'a str, max_len: usize) -> &'a str {
    if s.len() > max_len {
        &s[..max_len]
    } else {
        s
    }
}

fn main() {
    // --- first_word ---
    let sentence = "hello world from Rust";
    let word = first_word(sentence);
    println!("first_word('{sentence}') = '{word}'");

    // --- longest_snippet ---
    let long = "This is a very long string that should be truncated";
    let short = "Hi!";
    println!("longest_snippet long = '{}'", longest_snippet(long));
    println!("longest_snippet short = '{}'", longest_snippet(short));

    // --- trim_whitespace ---
    let padded = "  hello  ";
    let trimmed = trim_whitespace(padded);
    println!("trim_whitespace('{padded}') = '{trimmed}'");

    // --- TextHolder methods (Rule 3: &self) ---
    let doc = "line one\nline two\nline three";
    let holder = TextHolder { text: doc };
    println!("holder.content() = '{}'", holder.content());
    println!("holder.first_line() = '{}'", holder.first_line());

    // --- split_at_first ---
    let email = "user@example.com";
    if let Some((user, domain)) = split_at_first(email, '@') {
        println!("split_at_first('{email}'): user='{user}', domain='{domain}'");
    }

    // --- choose_prefix ---
    let data = "Rustacean";
    let prefix = choose_prefix(data, 4);
    println!("choose_prefix('{data}', 4) = '{prefix}'");
}
