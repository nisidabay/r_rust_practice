// 04_trait_bounds.rs — Generic functions with trait bounds
//
// Constrain generic type parameters with trait bounds.
// Syntax: <T: TraitName> or where T: TraitName

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("\"{}\" by {}", self.title, self.author)
    }
}

struct Tweet {
    username: String,
    text: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, &self.text[..self.text.len().min(50)])
    }
}

// --- Basic trait bound: T: Summary ---
fn print_summary<T: Summary>(item: T) {
    println!("Summary: {}", item.summarize());
}

// --- Multiple trait bounds ---
fn print_debug_summary<T: Summary + std::fmt::Debug>(item: T) {
    println!("Debug: {:?}", item);
    println!("Summary: {}", item.summarize());
}

// --- where clause (cleaner with many bounds) ---
fn notify<T>(item: T)
where
    T: Summary + std::fmt::Display,
{
    println!("Breaking news! {}", item.summarize());
    println!("Display: {}", item);
}

// --- Return type with trait bound ---
fn make_tweet() -> impl Summary {
    Tweet {
        username: "rust_daily".to_string(),
        text: "Rust traits are powerful!".to_string(),
    }
}

impl std::fmt::Debug for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Article")
            .field("title", &self.title)
            .field("author", &self.author)
            .finish()
    }
}

impl std::fmt::Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\" — {}", self.title, self.author)
    }
}

fn main() {
    let article = Article {
        title: "Rust Ownership Explained".to_string(),
        author: "Alice".to_string(),
    };

    // Basic bound
    print_summary(article);

    let tweet = Tweet {
        username: "bob".to_string(),
        text: "Just learned about trait bounds! #rust".to_string(),
    };

    // Multiple bounds (Debug + Summary)
    // Tweet doesn't impl Debug, so uncommenting this won't compile:
    // print_debug_summary(tweet);

    // where clause (need Display + Summary)
    let article2 = Article {
        title: "Zero-Cost Abstractions".to_string(),
        author: "Carol".to_string(),
    };
    notify(article2);

    // impl Trait return type
    let t = make_tweet();
    println!("Returned: {}", t.summarize());
}
