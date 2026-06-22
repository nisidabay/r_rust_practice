// 03_fn_traits.rs — Fn, FnMut, FnOnce: the three closure traits
//
// Every closure implements one or more of these traits. The compiler
// picks the MOST RESTRICTIVE one that the closure's body requires.

// Fn: can be called multiple times, doesn't mutate captured variables.
// This is the most permissive. Only reads from captures.
fn call_fn<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)  // can call f multiple times
}

// FnMut: can be called multiple times, CAN mutate captures.
// Requires &mut self to call.
fn call_fn_mut<F: FnMut(i32) -> i32>(mut f: F, x: i32) -> i32 {
    f(x)  // can call f multiple times, it mutates internal state
}

// FnOnce: can be called ONCE (consumes captures by value).
// Requires ownership to call.
fn call_fn_once<F: FnOnce(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)  // can only call once
}

fn main() {
    // Example 1: Fn — only reads captures
    let multiplier = 10;
    let doubler = |x: i32| x * multiplier;  // only reads, so Fn
    println!("Fn call: {}", call_fn(doubler, 5));
    println!("Fn call again: {}", call_fn(doubler, 7));

    // Example 2: FnMut — mutates captures
    let mut sum = 0;
    let mut adder = |x: i32| {
        sum += x;  // mutates, so FnMut, not Fn
        sum
    };
    println!("FnMut call: {}", call_fn_mut(adder, 10));
    // adder is consumed by call_fn_mut because it takes FnMut by value
    // Can't call again: println!("{}", call_fn_mut(adder, 5));

    // Example 3: FnOnce — consumes capture
    let owned = String::from("hello");
    let consumer = |x: i32| -> i32 {
        let _ = owned;  // consumes owned, so FnOnce
        x
    };
    println!("FnOnce call: {}", call_fn_once(consumer, 42));
    // consumer consumed — can't call again

    // Example 4: same closure can be multiple traits depending on usage
    let data = vec![1, 2, 3];
    let just_read = || data.len();          // Fn (only reads)
    println!("Fn via len: {}", just_read());
    println!("Fn via len again: {}", just_read());
    println!("data still available: {:?}", data);

    println!("\nClosure traits: Fn > FnMut > FnOnce — compiler picks the right one.");
}
