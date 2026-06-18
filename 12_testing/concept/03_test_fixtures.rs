// 03_test_fixtures.rs — Setup/teardown patterns, test helpers
//
// Rust does not have built-in setup/teardown hooks like xUnit frameworks.
// Instead, you use helper functions, Drop, or the same #[test] pattern.
//
// Run: rustc --edition 2021 --test 03_test_fixtures.rs && ./03_test_fixtures

use std::fs;
use std::path::Path;

// --- Helper function as a fixture ---
// A common pattern: call a function at the start of each test.

/// Creates a temporary directory and returns its path.
/// The directory is cleaned up when TempDir is dropped.
struct TempDir {
    path: String,
}

impl TempDir {
    fn new(prefix: &str) -> Self {
        let dir = format!("/tmp/rust_test_{prefix}_{}", std::process::id());
        let _ = fs::remove_dir_all(&dir); // clean up from previous runs
        fs::create_dir_all(&dir).expect("failed to create temp dir");
        TempDir { path: dir }
    }

    fn path(&self) -> &str {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

// --- Counter fixture ---
// A simple struct that sets up state for multiple tests.

struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn value(&self) -> i32 {
        self.count
    }
}

// --- Tests using the fixtures ---

#[test]
fn test_temp_dir_created() {
    let dir = TempDir::new("test_created");
    assert!(Path::new(dir.path()).exists(), "temp dir should exist");
}

#[test]
fn test_temp_dir_file_ops() {
    let dir = TempDir::new("file_ops");
    let file_path = format!("{}/hello.txt", dir.path());

    fs::write(&file_path, "Hello, test!").expect("write failed");
    assert!(Path::new(&file_path).exists());

    let content = fs::read_to_string(&file_path).expect("read failed");
    assert_eq!(content, "Hello, test!");
}

#[test]
fn test_counter_starts_at_zero() {
    let mut c = Counter::new();
    assert_eq!(c.value(), 0);
}

#[test]
fn test_counter_increment() {
    let mut c = Counter::new();
    c.increment();
    c.increment();
    c.increment();
    assert_eq!(c.value(), 3);
}

// --- Test helper modules ---
// You can extract shared helpers into a separate module:

#[cfg(test)]
mod helpers {
    /// A helper that creates a Vec of test data.
    pub fn test_data() -> Vec<i32> {
        vec![1, 2, 3, 4, 5]
    }

    /// A helper that checks if a number is even.
    pub fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
}

#[cfg(test)]
mod tests_using_helpers {
    use super::helpers;

    #[test]
    fn test_using_helper_data() {
        let data = helpers::test_data();
        assert_eq!(data.len(), 5);
        assert!(data.contains(&3));
    }

    #[test]
    fn test_using_helper_function() {
        assert!(helpers::is_even(4));
        assert!(!helpers::is_even(7));
    }
}
