// ex04_bonus_property.rs — Property-based testing (manual)
//
// Check properties across multiple random inputs in a loop.
// Properties:
//   1. reverse(reverse(x)) == x
//   2. reverse(x).len() == x.len()
//   3. reverse(empty) == empty
//
// Run: rustc --test --edition 2021 ex04_bonus_property.rs -o /tmp/test && /tmp/test

/// Reverse a vector
pub fn reverse<T: Clone>(v: &[T]) -> Vec<T> {
    let mut result = v.to_vec();
    result.reverse();
    result
}

/// Check if a string is a palindrome (property: reverse == original)
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let lower = cleaned.to_lowercase();
    lower.chars().eq(lower.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Property: reverse twice is original ---
    #[test]
    fn reverse_twice_is_original() {
        // Test with various inputs
        let inputs = vec![
            vec![1, 2, 3, 4, 5],
            vec![],
            vec![42],
            vec![1, 1, 1, 1],
            vec![10, 20, 30, 40, 50, 60],
        ];

        for input in inputs {
            let reversed = reverse(&input);
            let double_reversed = reverse(&reversed);
            assert_eq!(
                double_reversed, input,
                "reverse(reverse({:?})) should be {:?}, got {:?}",
                input, input, double_reversed
            );
        }
    }

    // --- Property: length preserved ---
    #[test]
    fn reverse_preserves_length() {
        let inputs = vec![
            vec![1, 2, 3],
            vec![],
            vec![5; 100],
            vec![-1, 0, 1, 2],
        ];

        for input in &inputs {
            let reversed = reverse(input);
            assert_eq!(
                reversed.len(),
                input.len(),
                "reverse({:?}).len() should be {}, got {}",
                input,
                input.len(),
                reversed.len()
            );
        }
    }

    // --- Property: reverse(empty) == empty ---
    #[test]
    fn reverse_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(reverse(&empty), empty);
        assert!(reverse(&empty).is_empty());
    }

    // --- Property-based: random inputs ---
    #[test]
    fn reverse_random_properties() {
        use std::collections::HashSet;

        // Generate several random-ish test cases
        let test_cases: Vec<Vec<i32>> = vec![
            vec![7, 3, 1, 9, 4],
            vec![-5, 10, -3, 8, 0, 2],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        ];

        for input in test_cases {
            // Property: first element becomes last
            if !input.is_empty() {
                let reversed = reverse(&input);
                assert_eq!(reversed[0], input[input.len() - 1]);
                assert_eq!(reversed[input.len() - 1], input[0]);
            }

            // Property: elements are the same (just reordered)
            let original_set: HashSet<i32> = input.iter().cloned().collect();
            let reversed_set: HashSet<i32> = reverse(&input).iter().cloned().collect();
            assert_eq!(original_set, reversed_set);
        }
    }

    // --- Property: palindrome properties ---
    #[test]
    fn palindrome_properties() {
        // A palindrome reversed is itself
        let palindromes = vec![
            "racecar",
            "A man, a plan, a canal: Panama",
            "madam",
            "level",
            "",       // empty string is a palindrome
            "a",
        ];

        for s in &palindromes {
            assert!(is_palindrome(s), "'{}' should be a palindrome", s);
        }

        let not_palindromes = vec![
            "hello",
            "rust",
            "abc",
        ];

        for s in &not_palindromes {
            assert!(!is_palindrome(s), "'{}' should NOT be a palindrome", s);
        }
    }

    // --- Property: repeated reversal in a loop ---
    #[test]
    fn reverse_many_times() {
        let original = vec![1, 2, 3, 4, 5];
        let mut current = original.clone();

        // Reversing twice should always give back the original
        for _ in 0..10 {
            current = reverse(&current);
        }

        assert_eq!(current, original);

        // Odd number of reversals = reversed
        current = original.clone();
        for _ in 0..7 {
            current = reverse(&current);
        }
        // 7 is odd, so result should be reversed once
        let expected: Vec<i32> = original.iter().rev().cloned().collect();
        assert_eq!(current, expected);
    }
}

fn main() {
    println!("reverse([1,2,3]) = {:?}", reverse(&[1, 2, 3]));
    println!("is_palindrome('racecar') = {}", is_palindrome("racecar"));
    println!("\nRun: rustc --test --edition 2021 ex04_bonus_property.rs -o /tmp/t && /tmp/t");
}
