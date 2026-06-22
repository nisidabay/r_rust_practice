// 01_trait_define.rs — Define a trait and implement it for multiple types
//
// Traits are Rust's version of interfaces. They define shared behavior
// that multiple types can implement.

// Define a trait
trait Summary {
    fn summarize(&self) -> String;
}

// --- Implement for a struct ---
struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("\"{}\" by {}", self.title, self.author)
    }
}

// --- Implement for a tuple struct ---
struct Tweet {
    username: String,
    text: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, &self.text[..self.text.len().min(50)])
    }
}

// --- Implement for a built-in type ---
impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("The number {}", self)
    }
}

fn main() {
    let article = Article {
        title: "Rust is Fast".to_string(),
        author: "Alice".to_string(),
        content: "Rust is a systems programming language...".to_string(),
    };

    let tweet = Tweet {
        username: "bob".to_string(),
        text: "Learning Rust traits! They're like interfaces but better. #rust".to_string(),
    };

    println!("Article: {}", article.summarize());
    println!("Tweet:   {}", tweet.summarize());
    println!("Number:  {}", 42.summarize());
}
