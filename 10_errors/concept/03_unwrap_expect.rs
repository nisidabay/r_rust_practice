fn main() {
    // unwrap and expect — quick ways to get the value OR panic.
    // DANGEROUS in production. Use only for:
    //   1. Prototypes / quick scripts
    //   2. Tests (where panicking on failure is desired)
    //   3. When you KNOW the operation can't fail (infallible)

    // --- unwrap ---
    let x: Option<i32> = Some(42);
    println!("unwrap: {}", x.unwrap());  // 42

    // let y: Option<i32> = None;
    // y.unwrap();  // PANICS — called `Option::unwrap()` on a `None` value

    // --- expect (like unwrap but with a custom message) ---
    let r: Result<i32, &str> = Ok(99);
    println!("expect: {}", r.expect("This should never fail"));  // 99

    // let e: Result<i32, &str> = Err("oops");
    // e.expect("Expected a number");  // PANICS with: "Expected a number: oops"

    // --- When unwrap is OK (infallible operations) ---
    // Some operations genuinely can't fail:
    let v = vec![1, 2, 3];
    let first = v.get(0).unwrap();  // We know index 0 exists
    println!("first: {}", first);

    // String parsing that we know is valid:
    let n = "42".parse::<i32>().unwrap();
    println!("parsed: {}", n);

    // --- When expect is better than unwrap ---
    // expect gives context when the panic happens
    let config_val = std::env::var("HOME").expect("HOME must be set");
    println!("HOME: {}", config_val);

    // The lesson: unwrap/expect is for prototyping and infallible cases.
    // For real code, use match, ? operator, or combinator methods.
    println!("\nBest practices:");
    println!("  Prototyping: unwrap/expect are fine (move fast)");
    println!("  Production:  use match, ?, or combinators");
    println!("  Tests:       unwrap/expect are preferred (fail fast)");
}
