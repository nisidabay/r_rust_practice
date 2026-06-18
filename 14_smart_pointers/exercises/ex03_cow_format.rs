// ex03_cow_format.rs — Use Cow<str> to avoid unnecessary string copies
//
// Write a function that formats a name for display. If the name already
// matches the desired format, return a borrowed Cow — no allocation.
// Only allocate (clone) when the format actually needs to change.

use std::borrow::Cow;

// TODO: Implement format_display_name(name: &str) -> Cow<str>
//
// Rules for the "display format":
// 1. If the name is empty, return "Guest" (allocated)
// 2. If the name is exactly 3 characters and all uppercase (e.g., "BOB"), 
//    return it as-is (borrowed) — it already looks like a nickname.
// 3. If the name has at least one underscore, replace underscores with spaces
//    and title-case it (allocated). E.g., "john_doe" -> "John Doe"
// 4. Otherwise, title-case the name (allocated if changed).
//    E.g., "alice" -> "Alice", but "Alice" stays borrowed.
//
// Return Cow::Borrowed when no allocation is needed, Cow::Owned otherwise.

// TODO: Helper function title_case(s: &str) -> String
// Capitalize the first letter of each word.

// TODO: Helper function is_uppercase(s: &str) -> bool
// Check if all alphabetic characters are uppercase.

fn main() {
    // Test cases
    let tests = vec![
        ("", "Guest", false),     // empty -> "Guest" (owned)
        ("BOB", "BOB", true),     // 3-char uppercase -> borrowed
        ("ALICE", "Alice", false), // long uppercase -> title cased (owned)
        ("alice", "Alice", false), // lowercase -> title cased (owned)
        ("Alice", "Alice", true),  // already correct -> borrowed
        ("john_doe", "John Doe", false), // underscores -> spaces + title (owned)
        ("jane_mary_smith", "Jane Mary Smith", false), // multiple underscores
        ("BOB_SMITH", "Bob Smith", false), // mixed underscores + uppercase
    ];

    for (input, expected, should_be_borrowed) in &tests {
        let result = format_display_name(input);
        assert_eq!(
            result.as_ref(),
            *expected,
            "Input: {:?}",
            input
        );
        assert_eq!(
            matches!(result, Cow::Borrowed(_)),
            *should_be_borrowed,
            "Input: {:?} — expected borrowed={}, got {}",
            input,
            should_be_borrowed,
            matches!(result, Cow::Borrowed(_))
        );
        println!(
            "'{}' -> '{}' (borrowed: {})",
            input,
            result,
            matches!(result, Cow::Borrowed(_))
        );
    }

    println!("All tests passed!");
}
