// `match` is Rust's powerful pattern-matching construct — it's like
// switch on steroids. Every arm must be exhaustive (cover all cases).

fn main() {
    // --- Match on integers ---
    // Classic switch-like usage: match an integer against literal values.
    let status_code = 404;
    let reason = match status_code {
        // Each arm is a pattern followed by `=>` and an expression.
        200 => "OK",
        301 | 302 => "Redirect",          // `|` matches multiple values
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        // `_` is the catch-all pattern (like `default` in C switch).
        // It matches anything; order matters — catch-all goes LAST.
        _ => "Unknown",
    };
    println!("Status {status_code}: {reason}");

    // --- Match with ranges ---
    // Patterns can include ranges with `..=` (inclusive).
    let score = 85;
    let grade = match score {
        // `..=` matches a closed range from low to high.
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        // Range from 0 up to but NOT including 60.
        0..=59 => "F",
        // Catch-all for out-of-range values.
        _ => "Invalid score",
    };
    println!("Score {score}: Grade {grade}");

    // --- Match with multiple patterns (|) ---
    // The `|` operator lets one arm handle several values.
    let day = 6; // Saturday
    let is_weekend = match day {
        1 | 2 | 3 | 4 | 5 => false,   // Weekdays
        6 | 7 => true,                  // Weekend
        _ => false,                     // Invalid day
    };
    println!("Day {day} is weekend: {is_weekend}");

    // --- Match must be exhaustive ---
    // The compiler forces every possible value to be covered.
    // Without the `_` catch-all, this would be a compile error.
    let value = 42;
    match value {
        0 => println!("zero"),
        1..=10 => println!("small"),
        // Without `_ => ...` this wouldn't compile.
        _ => println!("other"),
    }

    // --- Match as an expression (returning a value) ---
    // Like `if`, `match` is an expression that can be assigned.
    let num = 7;
    let description = match num {
        1 => "one",
        2 | 3 | 5 | 7 | 11 => "prime",
        4 | 6 | 8 | 9 | 10 => "composite",
        _ => "out of range",
    };
    println!("{num} is {description}");
}
