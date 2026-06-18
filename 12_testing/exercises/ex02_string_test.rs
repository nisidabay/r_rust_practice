// ex02_string_test.rs — Test string utilities: reverse, is_palindrome, count_words
//
// Implement string utility functions and write tests covering edge cases.
// Edge cases: empty strings, single chars, Unicode, whitespace, special chars.
//
// Run: rustc --edition 2021 --test ex02_string_test.rs && ./ex02_string_test

// --- Implement these functions ---

/// Reverses a string by characters (Unicode grapheme-aware).
fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// Checks if a string is a palindrome (ignoring case and non-alphanumeric chars).
fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let rev: String = cleaned.chars().rev().collect();
    cleaned == rev
}

/// Counts words in a string (whitespace-separated, trimming empty runs).
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    // --- reverse ---

    #[test]
    fn test_reverse_empty() {
        assert_eq!(reverse(""), "");
    }

    #[test]
    fn test_reverse_single_char() {
        assert_eq!(reverse("a"), "a");
    }

    #[test]
    fn test_reverse_word() {
        assert_eq!(reverse("hello"), "olleh");
    }

    #[test]
    fn test_reverse_sentence() {
        assert_eq!(reverse("race car"), "rac ecar");
    }

    #[test]
    fn test_reverse_palindrome() {
        assert_eq!(reverse("racecar"), "racecar");
    }

    #[test]
    fn test_reverse_unicode() {
        assert_eq!(reverse("¡Hola!"), "!aloH¡");
    }

    #[test]
    fn test_reverse_multi_byte() {
        // "🦀" is a 4-byte character
        assert_eq!(reverse("a🦀b"), "b🦀a");
    }

    #[test]
    fn test_reverse_numbers() {
        assert_eq!(reverse("12345"), "54321");
    }

    // --- is_palindrome ---

    #[test]
    fn test_is_palindrome_empty() {
        assert!(is_palindrome(""));
    }

    #[test]
    fn test_is_palindrome_single_char() {
        assert!(is_palindrome("a"));
    }

    #[test]
    fn test_is_palindrome_classic() {
        assert!(is_palindrome("racecar"));
    }

    #[test]
    fn test_is_palindrome_case_insensitive() {
        assert!(is_palindrome("RaceCar"));
    }

    #[test]
    fn test_is_palindrome_with_spaces() {
        assert!(is_palindrome("a man a plan a canal panama"));
    }

    #[test]
    fn test_is_palindrome_with_punctuation() {
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
    }

    #[test]
    fn test_is_palindrome_not() {
        assert!(!is_palindrome("hello"));
    }

    #[test]
    fn test_is_palindrome_not_close() {
        assert!(!is_palindrome("almost"));
    }

    #[test]
    fn test_is_palindrome_numbers() {
        assert!(is_palindrome("12321"));
        assert!(!is_palindrome("12345"));
    }

    // --- count_words ---

    #[test]
    fn test_count_words_empty() {
        assert_eq!(count_words(""), 0);
    }

    #[test]
    fn test_count_words_single() {
        assert_eq!(count_words("hello"), 1);
    }

    #[test]
    fn test_count_words_multiple() {
        assert_eq!(count_words("hello world rust"), 3);
    }

    #[test]
    fn test_count_words_with_extra_spaces() {
        assert_eq!(count_words("  hello   world  "), 2);
    }

    #[test]
    fn test_count_words_tabs_newlines() {
        assert_eq!(count_words("hello\tworld\nrust"), 3);
    }

    #[test]
    fn test_count_words_punctuation() {
        assert_eq!(count_words("hello, world! testing..."), 3);
    }

    #[test]
    fn test_count_words_only_whitespace() {
        assert_eq!(count_words("   \t  \n  "), 0);
    }

    #[test]
    fn test_count_words_unicode() {
        assert_eq!(count_words("¡Hola! ¿Cómo estás?"), 3);
    }
}
