// 03_unwrap_expect — unwrap, expect, and why they're dangerous in production
//
// unwrap() and expect() are convenience methods on Result and Option that
// return the inner Ok/Some value OR panic if there's an Err/None.
//
// In production code they're considered a code smell — prefer explicit
// error handling. They're fine in prototypes, examples, and tests.

fn main() {
    // --- 1. unwrap on Ok returns the value ---
    let ok: Result<i32, &str> = Ok(10);
    let val = ok.unwrap();
    println!("unwrap(Ok(10)) = {val}");

    // --- 2. unwrap on Err panics ---
    // The next line would crash the program:
    // let bad: Result<i32, &str> = Err("oops");
    // bad.unwrap(); // PANIC!

    // We can catch it to see the message without crashing:
    let bad: Result<i32, &str> = Err("oops");
    let caught = std::panic::catch_unwind(|| bad.unwrap());
    println!("unwrap(Err) panicked: {:?}", caught.is_err());

    // --- 3. expect gives a custom panic message ---
    // expect is unwrap + a descriptive message.
    let val = Ok::<i32, &str>(42).expect("This should never fail");
    println!("expect(Ok(42)) = {val}");

    // On failure:
    // let _ = Err::<i32, &str>("disk full").expect("Failed to read config");
    // PANIC with: "Failed to read config: \"disk full\""

    // --- 4. Why unwrap is dangerous ---
    // Consider: a function that returns Result, and you unwrap it.
    // If the function later changes and starts returning Err, your
    // code panics at runtime — the compiler gives no warning.
    //
    // Prefer:
    //   match / if let   — for local handling
    //   ?                 — to propagate to the caller
    //   unwrap_or / unwrap_or_default — for fallback defaults

    // --- 5. Safer alternatives ---
    fn parse_or_default(s: &str) -> i32 {
        s.parse::<i32>().unwrap_or(0) // fallback, no panic
    }

    println!("parse_or_default(\"42\") = {}", parse_or_default("42"));
    println!("parse_or_default(\"abc\") = {}", parse_or_default("abc"));

    // unwrap_or_else — compute fallback lazily
    fn get_env_or(key: &str, fallback: i32) -> i32 {
        std::env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(fallback)
    }
    println!(
        "get_env_or(\"UNSET_VAR\", 99) = {}",
        get_env_or("UNSET_VAR", 99)
    );

    // unwrap_or_default — uses Default trait
    let none: Option<i32> = None;
    println!("None.unwrap_or_default() = {}", none.unwrap_or_default());

    println!("--- 03_unwrap_expect done ---");
}
