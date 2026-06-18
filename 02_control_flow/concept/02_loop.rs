// `loop` creates an infinite loop — it never stops unless we `break` out.
// It's the simplest looping construct and the most flexible.

fn main() {
    // --- Infinite loop with break ---
    // A bare `loop { ... }` repeats forever.
    // `break` exits the loop immediately.
    let mut count = 0;
    loop {
        count += 1;
        if count >= 5 {
            // Exit the loop — without this we'd run forever.
            break;
        }
    }
    println!("Looped {count} times before break");

    // --- loop is an expression that can return a value ---
    // `break` can carry a value out of the loop.
    // This is unique to Rust — most languages' loops are statements.
    let mut n = 0;
    let result = loop {
        n += 1;
        if n == 10 {
            // Break with a value; assigned to `result`.
            break n * 2;
        }
    };
    // result = 20 (10 * 2)
    println!("loop returned: {result}");

    // --- continue: skip to next iteration ---
    // `continue` jumps to the top of the loop, skipping the rest of the body.
    let mut sum = 0;
    for i in 1..=10 {
        if i % 2 == 0 {
            // Skip even numbers — they don't contribute to sum here.
            continue;
        }
        sum += i;
    }
    // sum = 1 + 3 + 5 + 7 + 9 = 25
    println!("Sum of odd numbers 1..10: {sum}");

    // --- Nested loops ---
    // Each `break` / `continue` applies to the innermost loop by default.
    let mut found = false;
    for x in 1..=3 {
        for y in 1..=3 {
            if x * y == 6 {
                found = true;
                // This only breaks the inner `y` loop.
                break;
            }
        }
        // The outer `x` loop continues here unless we check `found`.
        if found {
            break;
        }
    }
    println!("Found pair whose product is 6: {found}");
}
