// html_builder — A safe HTML builder using lifetimes.
//
// Build HTML by composing Tag structs that borrow their tag names and attributes
// from the caller. No string copies needed for tag names — everything is a &str
// reference tied to the data's lifetime.
//
// Usage:
//   cargo run -- div "Hello, world!"
//   cargo run -- a "Click here" --href "https://example.com"
//   cargo run -- --help
//
// Demonstrates:
//   - Struct lifetime parameters (Tag<'a, 'b>)
//   - Multiple lifetime parameters for different references
//   - Elision rules on methods (&self → output lifetime)
//   - The 'static lifetime for built-in tag names

use std::env;

// ─── Core types ─────────────────────────────────────────────────────────────

/// An attribute in an HTML tag — the name and value are borrowed references.
#[derive(Debug)]
struct Attribute<'a, 'b> {
    name: &'a str,
    value: &'b str,
}

/// An HTML tag with borrowed name and optional attributes.
/// 'a is the lifetime of the tag name.
/// 'b is the lifetime of the attribute values.
#[derive(Debug)]
struct Tag<'a, 'b> {
    name: &'a str,
    attributes: Vec<Attribute<'a, 'b>>,
}

impl<'a, 'b> Tag<'a, 'b> {
    /// Create a new tag with the given name.
    fn new(name: &'a str) -> Self {
        Tag {
            name,
            attributes: Vec::new(),
        }
    }

    /// Add an attribute to the tag.
    fn attr(mut self, name: &'a str, value: &'b str) -> Self {
        self.attributes.push(Attribute { name, value });
        self
    }

    /// Render the opening tag: <name attr="value">
    fn open(&self) -> String {
        let mut out = String::from("<");
        out.push_str(self.name);
        for attr in &self.attributes {
            out.push(' ');
            out.push_str(attr.name);
            out.push_str("=\"");
            out.push_str(attr.value);
            out.push('"');
        }
        out.push('>');
        out
    }

    /// Render the closing tag: </name>
    fn close(&self) -> String {
        format!("</{}>", self.name)
    }

    /// Render a full element: <name>content</name>
    fn wrap(&self, content: &str) -> String {
        format!("{}{}{}", self.open(), content, self.close())
    }
}

// ─── Static helpers ─────────────────────────────────────────────────────────

/// Tags that use &'static str — no runtime allocation for tag names.
const VOID_TAGS: &[&str] = &["br", "hr", "img", "input", "meta", "link"];

/// Check if a tag name is a void element (no closing tag needed).
fn is_void_tag(name: &str) -> bool {
    VOID_TAGS.contains(&name)
}

/// Escape HTML special characters in a string.
/// Returns an owned String (the escaped version).
fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
    out
}

// ─── Help ───────────────────────────────────────────────────────────────────

fn print_help() {
    println!("html_builder 0.1.0");
    println!("A safe HTML builder using lifetimes");
    println!();
    println!("USAGE:");
    println!("  cargo run -- <tag> <content> [--<attr> <value>]...");
    println!("  cargo run -- --help");
    println!();
    println!("ARGUMENTS:");
    println!("  <tag>        HTML tag name (e.g., div, p, a, span)");
    println!("  <content>    Content to wrap in the tag");
    println!();
    println!("OPTIONS:");
    println!("  --<attr> <value>  Add an HTML attribute (e.g., --href url)");
    println!("  --help           Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("  cargo run -- div \"Hello, world!\"");
    println!("  cargo run -- a \"Click here\" --href \"https://example.com\"");
    println!("  cargo run -- img \"\" --src \"image.png\" --alt \"An image\"");
    println!();
    println!("HOW IT DEMONSTRATES LIFETIMES:");
    println!("  1. Tag struct borrows name and attribute values (&'a str / &'b str)");
    println!("  2. No String allocation for tag names (static &str where possible)");
    println!("  3. All borrowed data must outlive the Tag struct");
    println!("  4. Methods use lifetime elision (= self lifetime for output)");
}

// ─── Main ───────────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = env::args().collect();

    // Show help
    if args.len() <= 1 || (args.len() == 2 && args[1] == "--help") {
        print_help();
        return;
    }

    if args.len() < 3 {
        eprintln!("Error: expected at least 2 arguments: <tag> <content>");
        eprintln!("Usage: cargo run -- <tag> <content> [--<attr> <value>]...");
        std::process::exit(1);
    }

    // Parse arguments
    let tag_name: &str = &args[1];
    let content: &str = &args[2];

    // Parse attributes: --key value pairs from args[3..]
    let mut attr_names: Vec<&str> = Vec::new();
    let mut attr_values: Vec<&str> = Vec::new();

    let mut i = 3;
    while i < args.len() {
        if args[i].starts_with("--") {
            let attr_name = &args[i][2..]; // strip "--"
            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                attr_names.push(attr_name);
                attr_values.push(&args[i + 1]);
                i += 2;
            } else {
                // Boolean attribute (no value), e.g., --disabled
                attr_names.push(attr_name);
                attr_values.push("");
                i += 1;
            }
        } else {
            eprintln!("Warning: unexpected argument '{}' (expected --attr)", args[i]);
            i += 1;
        }
    }

    // Build the tag using our lifetime-aware API
    // Both the tag name and attribute references borrow args's data
    let mut tag: Tag<'_, '_> = Tag::new(tag_name);

    for (name, value) in attr_names.iter().zip(attr_values.iter()) {
        tag = tag.attr(name, value);
    }

    // Render
    if is_void_tag(tag_name) {
        // Void elements just render the opening tag (or self-closing)
        println!("{}", tag.open());
    } else {
        let escaped = escape_html(content);
        println!("{}", tag.wrap(&escaped));
    }

    // Demonstrate that the tag borrows from args — args lives to end of main
    // so the borrows are always valid. If we tried to use `tag` after moving
    // `args`, the compiler would catch it.
}
