// 02_test_module.rs — #[cfg(test)] mod tests { use super::*; }
//
// Organize tests into a module. #[cfg(test)] ensures the module
// is only compiled when testing.
//
// Run with: rustc --test --edition 2021 02_test_module.rs -o /tmp/t2 && /tmp/t2

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

// Test module — only compiled during `cargo test` or `rustc --test`
#[cfg(test)]
mod tests {
    // Bring parent module functions into scope
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(multiply(0, 5), 0);
        assert_eq!(multiply(-2, 3), -6);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_multiply_large() {
        assert_eq!(multiply(1000, 1000), 1_000_000);
    }
}

// This module is NOT compiled unless we're testing
// The #[cfg(test)] attribute is the gate

fn main() {
    println!("multiply(3, 4) = {}", multiply(3, 4));
    println!("factorial(5) = {}", factorial(5));
    println!("\nTests are in a separate module with #[cfg(test)]");
    println!("Run: rustc --test --edition 2021 02_test_module.rs -o /tmp/t2 && /tmp/t2");
}
