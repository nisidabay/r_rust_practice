// `if let` is a concise way to match a single pattern and ignore the rest.
// Use it when you only care about one variant of an enum (like Option or Result)
// and don't need the full exhaustiveness of match.

fn main() {
    // --- if let with Option ---
    // Without if let, checking one variant requires a verbose match.
    let maybe_number: Option<i32> = Some(42);

    // Verbose way (use if let instead when only Some matters):
    match &maybe_number {
        Some(n) => println!("Got a number: {n}"),
        None => {} // Must handle None even if we don't care.
    }

    // Concise way with if let — only handle the Some case.
    if let Some(n) = maybe_number {
        // This block runs ONLY if maybe_number is Some.
        // The value is destructured into `n`.
        println!("Got a number (if let): {n}");
    }
    // No else needed — if maybe_number is None, nothing happens.

    // --- if let with else ---
    // Add `else` to handle the non-matching case.
    let maybe_value: Option<i32> = None;
    if let Some(val) = maybe_value {
        println!("Value is {val}");
    } else {
        println!("No value present");
    }

    // --- if let with Result ---
    // Result works the same way: match Ok, ignore Err (or vice versa).
    let result: Result<i32, &str> = Ok(100);
    if let Ok(value) = result {
        println!("Operation succeeded with: {value}");
    } else {
        println!("Operation failed");
    }

    // --- if let with custom enum ---
    #[allow(dead_code)] // Used for demonstration only — not all variants exercised.
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    let coin = Coin::Dime;

    // Only care about one variant.
    if let Coin::Dime = coin {
        println!("Found a dime!");
    }

    // We can also use guards (extra conditions) with if let via
    // combining with a regular if.
    let some_num: Option<i32> = Some(7);
    if let Some(x) = some_num {
        if x > 5 {
            println!("{x} is greater than 5");
        }
    }
    // Shorthand: combine if let with a boolean expression on the same line
    // isn't directly supported; nest or use match with guards.

    // --- When to use if let vs match ---
    // USE if let when:
    //   - You only care about ONE variant
    //   - You want to destructure that one variant concisely
    // USE match when:
    //   - You need to handle MULTIPLE variants
    //   - You need exhaustiveness checking (compiler warns about missing arms)
    //   - You need guards (extra `if` conditions per arm)

    // Example: match is better here because we handle both variants.
    let opt: Option<i32> = Some(5);
    match opt {
        Some(x) if x < 0 => println!("Negative"),
        Some(_) => println!("Non-negative"),
        None => println!("Nothing"),
    }
}
