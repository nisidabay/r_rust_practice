fn main() {
    // if let — concise matching when you only care about ONE variant
    // Full match is safer (exhaustive). if let is convenient but skips the rest.

    let value: Option<i32> = Some(42);

    // Instead of:
    // match value { Some(v) => println!("{}", v), _ => {} }
    // Use if let:
    if let Some(v) = value {
        println!("Found value: {}", v);
    }

    // if let with else — handle the "matched" and "didn't match" cases
    let none_val: Option<i32> = None;
    if let Some(v) = none_val {
        println!("Found: {}", v);
    } else {
        println!("None value — no match");
    }

    // while let — loop while a pattern matches
    // Common pattern: draining an iterator of Options
    let mut stack = vec![1, 2, 3, 4, 5];

    println!("\nPopping from stack:");
    while let Some(top) = stack.pop() {
        // pop() returns Option<T> — Some(value) or None when empty
        // while let keeps going until pop() returns None
        println!("  popped: {}", top);
    }
    println!("Stack is now empty");

    // --- Practical: parsing a series of key=value pairs ---
    let pairs = vec!["name=Alice", "age=30", "badformat", "city=NYC"];

    println!("\nParsing key=value pairs:");
    for pair in &pairs {
        // split_once splits at first '=', returns Option<(&str, &str)>
        if let Some((key, val)) = pair.split_once('=') {
            println!("  {}: {}", key, val);
        } else {
            println!("  (skipped: no '=' in '{}')", pair);
        }
    }
}
