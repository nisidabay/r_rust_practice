// 07_mocking.rs — Basic mock patterns with enums and traits
//
// Rust doesn't have a built-in mock framework, but you can mock easily
// with enums, traits, and generics. This file shows three patterns.
//
// Run: rustc --edition 2021 --test 07_mocking.rs && ./07_mocking

// --- Pattern 1: Trait-based mocking ---
// Define a trait for your dependency, implement it for both real and mock.

/// A trait for sending notifications.
trait Notifier {
    fn send(&mut self, message: &str) -> Result<(), String>;
}

/// Real implementation — would send an actual notification.
struct EmailNotifier {
    server: String,
}

impl Notifier for EmailNotifier {
    fn send(&mut self, message: &str) -> Result<(), String> {
        // In real code: connect to SMTP server and send
        println!("[EMAIL] Sending to {}: {}", self.server, message);
        Ok(())
    }
}

/// Mock implementation — records messages for verification.
#[derive(Debug)]
struct MockNotifier {
    messages: Vec<String>,
    should_fail: bool,
}

impl MockNotifier {
    fn new(should_fail: bool) -> Self {
        MockNotifier {
            messages: Vec::new(),
            should_fail,
        }
    }
}

impl Notifier for MockNotifier {
    fn send(&mut self, message: &str) -> Result<(), String> {
        if self.should_fail {
            return Err("mock failure".into());
        }
        self.messages.push(message.to_string());
        Ok(())
    }
}

/// The function we want to test — generic over Notifier.
fn notify_users<N: Notifier>(notifier: &mut N, event: &str) -> Result<(), String> {
    notifier.send(&format!("ALERT: {event}"))?;
    notifier.send(&format!("REMINDER: {event} action required"))?;
    Ok(())
}

// --- Pattern 2: Enum-based mocking (simpler, no trait needed) ---

#[derive(Debug, PartialEq)]
enum DbResult {
    UserFound { id: u32, name: String },
    NotFound,
    Error(String),
}

/// A database client — could be real or mock.
fn fetch_user(client: &str, user_id: u32) -> DbResult {
    // In real code, query a database
    match (client, user_id) {
        ("mock", 1) => DbResult::UserFound {
            id: 1,
            name: "Alice".into(),
        },
        ("mock", 99) => DbResult::NotFound,
        ("mock", _) => DbResult::Error("connection timeout".into()),
        _ => DbResult::Error("unknown client".into()),
    }
}

/// Process a user — uses fetch_user internally.
fn process_user(client: &str, user_id: u32) -> Result<String, String> {
    match fetch_user(client, user_id) {
        DbResult::UserFound { name, .. } => Ok(format!("Hello, {name}!")),
        DbResult::NotFound => Err("user not found".into()),
        DbResult::Error(msg) => Err(msg),
    }
}

// --- Pattern 3: Closure-based injection (lightweight) ---

/// Greets a user. Accepts a greeting function for testability.
fn greet_user<F>(name: &str, greeter: F) -> String
where
    F: Fn(&str) -> String,
{
    greeter(name)
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    // --- Trait-based mock tests ---

    #[test]
    fn test_notify_users_success() {
        let mut mock = MockNotifier::new(false);
        let result = notify_users(&mut mock, "deploy");
        assert!(result.is_ok());
        assert_eq!(mock.messages.len(), 2);
        assert_eq!(mock.messages[0], "ALERT: deploy");
        assert_eq!(mock.messages[1], "REMINDER: deploy action required");
    }

    #[test]
    fn test_notify_users_failure() {
        let mut mock = MockNotifier::new(true);
        let result = notify_users(&mut mock, "deploy");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "mock failure");
    }

    // --- Enum-based mock tests ---

    #[test]
    fn test_fetch_user_found() {
        let result = fetch_user("mock", 1);
        assert_eq!(
            result,
            DbResult::UserFound {
                id: 1,
                name: "Alice".into()
            }
        );
    }

    #[test]
    fn test_fetch_user_not_found() {
        assert_eq!(fetch_user("mock", 99), DbResult::NotFound);
    }

    #[test]
    fn test_fetch_user_error() {
        match fetch_user("mock", 42) {
            DbResult::Error(_) => {} // expected
            _ => panic!("expected error"),
        }
    }

    #[test]
    fn test_process_user_found() {
        assert_eq!(
            process_user("mock", 1).unwrap(),
            "Hello, Alice!"
        );
    }

    #[test]
    fn test_process_user_not_found() {
        assert!(process_user("mock", 99).is_err());
    }

    // --- Closure injection tests ---

    #[test]
    fn test_greet_user_custom() {
        // Inject a mock greeting function
        let result = greet_user("Alice", |name| format!("Hi, {name}!"));
        assert_eq!(result, "Hi, Alice!");
    }

    #[test]
    fn test_greet_user_real() {
        // Use a real greeting function
        fn formal_greet(name: &str) -> String {
            format!("Dear {name},")
        }
        let result = greet_user("Bob", formal_greet);
        assert_eq!(result, "Dear Bob,");
    }
}
