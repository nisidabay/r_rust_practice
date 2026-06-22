// ex02_string.rs — fn truncate(s: &str, max: usize) -> String
//
// Tests for: empty, under max, over max, exact, Unicode boundary.
//
// Run: rustc --test --edition 2021 ex02_string.rs -o /tmp/test && /tmp/test

/// Truncate a string to at most `max` characters (not bytes).
/// If truncated, appends "..." at the end (counted in the max).
pub fn truncate(s: &str, max: usize) -> String {
    if max >= s.chars().count() {
        return s.to_string();
    }

    if max <= 3 {
        // Not enough space for "..."
        return s.chars().take(max).collect();
    }

    // Need to account for the "..." suffix
    let trunc_len = max - 3;
    let truncated: String = s.chars().take(trunc_len).collect();
    format!("{}...", truncated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(truncate("", 10), "");
    }

    #[test]
    fn under_max() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn over_max() {
        assert_eq!(truncate("hello world", 8), "hello...");
    }

    #[test]
    fn exact_length() {
        assert_eq!(truncate("hello", 5), "hello");
    }

    #[test]
    fn very_short_max() {
        assert_eq!(truncate("hello world", 3), "hel");
        assert_eq!(truncate("hello world", 2), "he");
        assert_eq!(truncate("hello world", 1), "h");
    }

    #[test]
    fn max_is_three() {
        assert_eq!(truncate("hello world", 3), "hel");
    }

    #[test]
    fn max_is_four() {
        assert_eq!(truncate("hello world", 4), "h...");
    }

    #[test]
    fn single_char() {
        assert_eq!(truncate("a", 1), "a");
        assert_eq!(truncate("a", 0), "");
    }

    #[test]
    fn unicode_boundary() {
        // héllo = 5 chars. max must be < 5 to truncate
        assert_eq!(truncate("héllo", 4), "h..."); // 1 + 3 = 4
        assert_eq!(truncate("héllo", 5), "héllo"); // 5 >= 5 -> no trunc
        assert_eq!(truncate("héllo", 3), "hél"); // max <= 3, no dots
    }

    #[test]
    fn unicode_multibyte() {
        // 5 emoji = 5 chars. max=4 < 5 -> truncates
        assert_eq!(truncate("🦀🦀🦀🦀🦀", 4), "🦀..."); // 1 + 3 = 4
        assert_eq!(truncate("🦀🦀🦀🦀🦀", 6), "🦀🦀🦀🦀🦀"); // 6 >= 5 -> no trunc
    }

    #[test]
    fn unicode_exact() {
        assert_eq!(truncate("café", 4), "café");
    }

    #[test]
    fn unicode_truncated() {
        assert_eq!(truncate("café au lait", 7), "café...");
        assert_eq!(truncate("café au lait", 6), "caf...");
    }

    #[test]
    fn string_shorter_than_max() {
        assert_eq!(truncate("hi", 100), "hi");
    }

    #[test]
    fn max_zero() {
        assert_eq!(truncate("anything", 0), "");
    }
}

fn main() {
    println!("truncate('hello world', 8) = '{}'", truncate("hello world", 8));
    println!("truncate('café au lait', 6) = '{}'", truncate("café au lait", 6));
    println!("\nRun: rustc --test --edition 2021 ex02_string.rs -o /tmp/t && /tmp/t");
}
