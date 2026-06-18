// Option<T> represents a value that may or may not be present.
// It's defined as: enum Option<T> { Some(T), None }
// This eliminates null pointer errors — the compiler forces you to handle both cases.

fn main() {
    // --- Creating Option values ---
    let some_number = Some(5);
    let some_string = Some(String::from("hello"));
    let absent_number: Option<i32> = None;  // type annotation required for None

    println!("some_number: {:?}", some_number);
    println!("absent_number: {:?}", absent_number);

    // --- Option is NOT the same as the inner type ---
    let x: i32 = 5;
    let y: Option<i32> = Some(10);

    // This won't compile:
    // let sum = x + y;  // ERROR: no implementation for `i32 + Option<i32>`
    // Must convert first.

    // SAFE way: match on Option to extract the value.
    let sum = match y {
        Some(val) => x + val,
        None => x,  // handle absence gracefully
    };
    println!("Sum (safe): {sum}");

    // --- Common Option methods ---

    // unwrap — panics if None
    let z = Some(42);
    println!("Unwrapped: {}", z.unwrap());

    // expect — panics with a custom message
    let w = Some(100);
    println!("Expected: {}", w.expect("value should be present"));

    // Let's make a None value to see the panic (commented out to keep the file running):
    // let nothing: Option<i32> = None;
    // nothing.unwrap();  // panics: called `Option::unwrap()` on a `None` value

    // --- is_some / is_none ---
    let val = Some(7);
    if val.is_some() {
        println!("Value exists: {:?}", val);
    }

    // --- map and unwrap_or ---
    let a: Option<i32> = Some(10);
    let b: Option<i32> = None;

    // map transforms the inner value if Some, does nothing if None.
    let doubled_a = a.map(|n| n * 2);
    let doubled_b = b.map(|n| n * 2);
    println!("Doubled Some: {:?}, Doubled None: {:?}", doubled_a, doubled_b);

    // unwrap_or provides a default.
    println!("a or 0: {}, b or 0: {}", a.unwrap_or(0), b.unwrap_or(0));

    // --- Real-world: safe division ---
    fn safe_divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }

    let result = safe_divide(10.0, 2.0);
    match result {
        Some(val) => println!("10 / 2 = {val}"),
        None => println!("Division by zero!"),
    }

    let result = safe_divide(10.0, 0.0);
    match result {
        Some(val) => println!("10 / 0 = {val}"),
        None => println!("Division by zero!"),
    }
}
