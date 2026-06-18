// `while` repeats a block as long as a boolean condition is true.
// `while let` combines pattern matching with looping — it runs while
// a pattern match succeeds.

fn main() {
    // --- Basic while loop ---
    // Keep running while the condition evaluates to true.
    let mut fuel = 100;
    while fuel > 0 {
        // Burn some fuel each iteration.
        fuel -= 7;
        // Clamp at 0 so we don't go negative — `while` checks only at
        // the top of the loop, not mid-iteration.
        if fuel < 0 {
            fuel = 0;
        }
        println!("Fuel remaining: {fuel}");
    }
    println!("Out of fuel!");

    // --- while with a counter ---
    // Classic counting loop using while.
    let mut i = 0;
    while i < 5 {
        println!("while iteration: {i}");
        i += 1;
    }
    // i is still accessible here (unlike some languages' for-loop variables).

    // --- while let ---
    // `while let` destructures an Option (or Result, or any pattern)
    // as long as matches succeed. When the pattern fails, the loop exits.
    let mut numbers: Vec<i32> = vec![10, 20, 30, 40];
    // `pop` returns Option<T> — Some(val) if non-empty, None if empty.
    while let Some(top) = numbers.pop() {
        // This loops while pop() returns Some; stops when pop() returns None.
        println!("Popped: {top}");
    }
    println!("Vector is now empty");

    // --- while let with a custom iterator ---
    // Another common pattern: advance through an iterator manually.
    let words = ["hello", "world", "from", "rust"];
    let mut iter = words.iter();
    // `iter.next()` returns `Option<&&str>` — Some while elements remain.
    while let Some(word) = iter.next() {
        println!("Word: {word}");
    }

    // --- while let with countdown ---
    // `while let` can destructure many types, not just Option.
    // Here we use a simple manual counter wrapped in an Option.
    let mut countdown: Option<i32> = Some(3);
    while let Some(val) = countdown {
        if val > 0 {
            println!("T-minus {val}");
            countdown = Some(val - 1);
        } else {
            println!("Liftoff!");
            countdown = None; // Setting to None ends the loop on next check.
        }
    }
}
