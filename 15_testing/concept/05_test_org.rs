// 05_test_org.rs — Unit vs integration tests. tests/ directory.
//
// Unit tests: in the same file/module as the code. Test private functions.
// Integration tests: in tests/ directory. Test the public API only.
//
// This file shows unit tests. Integration tests go in tests/*.rs and
// require a lib.rs (not just main.rs).
//
// Run with: This is a conceptual overview file.
// Compile and run to see the explanation.

// ---- Public API ----
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn is_palindrome(s: &str) -> bool {
    let s: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let lower = s.to_lowercase();
    lower.chars().eq(lower.chars().rev())
}

// ---- Private helper (testable via unit tests) ----
fn clean_text(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect()
}

// ---- Unit tests (can test private functions) ----
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
        assert_eq!(greet(""), "Hello, !");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(!is_palindrome("hello"));
    }

    // Unit tests can test private functions!
    #[test]
    fn test_clean_text() {
        assert_eq!(clean_text("hello, world!"), "hello world");
        assert_eq!(clean_text("  spaces  "), "  spaces  ");
    }
}

fn main() {
    println!("--- Test Organization ---");
    println!();
    println!("Unit tests        (this file):   #[cfg(test)] mod tests");
    println!("  - Test private functions");
    println!("  - Test internal implementation details");
    println!("  - Run with: cargo test (or rustc --test)");
    println!();
    println!("Integration tests  (tests/ dir):  tests/my_test.rs");
    println!("  - Test public API only");
    println!("  - Treat your library as an external consumer");
    println!("  - Need a lib.rs (not just main.rs)");
    println!("  - Run with: cargo test");
    println!();
    println!("Each tests/*.rs file is its own crate.");
    println!("Import via: use my_crate::function;");
}
