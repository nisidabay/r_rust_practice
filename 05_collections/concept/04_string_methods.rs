// 04_string_methods.rs — split, trim, contains, replace, chars, bytes
//
// String is the workhorse of text processing. First 5 lines: split, trim,
// contains, replace, walk chars.

fn main() {
    let text = "  hello, world!  ";

    // 1. trim — strip leading/trailing whitespace
    let trimmed = text.trim();
    println!("trimmed: '{trimmed}'");

    // 2. split — returns an iterator over &str slices
    let parts: Vec<&str> = "a,b,c".split(',').collect();
    println!("split: {parts:?}");

    // 3. contains — substring search
    println!("contains 'world'? {}", trimmed.contains("world"));

    // 4. replace — all occurrences
    let updated = trimmed.replace("world", "Rust");
    println!("replaced: {updated}");

    // 5. chars — iterate over Unicode scalar values
    for c in "Hi!".chars() {
        print!("[{c}]");
    }
    println!();

    // 6. bytes — raw UTF-8 bytes
    let greeting = "héllo";
    println!("UTF-8 bytes: {:?}", greeting.as_bytes());

    // 7. lines — split on newlines
    let poem = "line1\nline2\nline3";
    for (i, line) in poem.lines().enumerate() {
        println!("{i}: {line}");
    }

    // 8. starts_with / ends_with
    let filename = "main.rs";
    println!("starts with 'main'? {}", filename.starts_with("main"));
    println!("ends with '.rs'? {}", filename.ends_with(".rs"));

    // 9. to_uppercase / to_lowercase
    let shout = "hello".to_uppercase();
    println!("{shout}");

    // 10. find — first index of a substring
    if let Some(pos) = "abcdef".find("cd") {
        println!("'cd' starts at index {pos}");
    }
}
