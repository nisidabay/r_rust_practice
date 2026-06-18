// 03_fn_fnt_mut.rs — The three closure traits: Fn, FnMut, FnOnce
//
// Every closure implements one or more of the three closure traits:
//   FnOnce    — can be called once (takes ownership of captures)
//   FnMut     — can be called multiple times, may mutate captures
//   Fn        — can be called multiple times, only reads captures
//
// The compiler infers which traits a closure implements based on how
// it captures variables from its environment.

fn main() {
    // --- 1. Fn — immutable borrow, callable many times ---
    let x = 10;
    let add_x = |a: i32| a + x; // only reads x, so it's Fn
    // Fn is a subtrait of FnMut, which is a subtrait of FnOnce.
    // So Fn implements all three.
    println!("add_x(5) = {}", add_x(5));
    println!("add_x(3) = {}", add_x(3)); // Call again — no problem

    // --- 2. FnMut — mutable borrow, callable many times ---
    let mut sum = 0;
    let mut accumulate = |n: i32| {
        sum += n; // mutates sum via &mut, so it's FnMut
    };
    accumulate(5);
    accumulate(10);
    accumulate(3);
    // println!("{sum}"); // ERROR: still borrowed by accumulate
    drop(accumulate);
    println!("Sum after accumulates: {sum}");

    // --- 3. FnOnce — consumes captures, callable once ---
    let name = String::from("Bob");
    let consume = || {
        // Takes ownership of `name` by moving it
        drop(name); // actually consumes it
        println!("Consumed and dropped name");
    };
    consume();
    // consume(); // ERROR: closure can only be called once (FnOnce)

    // --- 4. Calling a generic function with different closures ---

    // A function that accepts Fn
    fn call_fn<F: Fn(i32) -> i32>(f: F, val: i32) -> i32 {
        f(val) // can call multiple times
    }

    let multiplier = 3;
    let triple = |x| x * multiplier; // Fn
    println!("triple(7) = {}", call_fn(triple, 7));

    // A function that accepts FnMut
    fn call_fn_mut<F: FnMut(i32) -> i32>(mut f: F, val: i32) -> i32 {
        f(val)
    }

    let mut acc = 10;
    let mut add_acc = |x| {
        acc += x; // FnMut
        acc
    };
    println!("add_acc(5) = {}", call_fn_mut(&mut add_acc, 5));

    // A function that accepts FnOnce
    fn call_fn_once<F: FnOnce(i32) -> i32>(f: F, val: i32) -> i32 {
        f(val) // called once
    }

    let data = vec![1, 2, 3];
    // FnOnce closure that takes the ignored argument and sums owned data
    let sum_vec = move |_: i32| data.into_iter().sum::<i32>();
    println!("sum_vec() = {}", call_fn_once(sum_vec, 0));

    // --- 5. Closures as function pointers ---
    // A closure that captures nothing can be coerced to a fn pointer.
    fn apply_fn_ptr(f: fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }

    let square_fn = |x: i32| x * x; // no captures → fn pointer eligible
    println!("square via fn ptr: {}", apply_fn_ptr(square_fn, 6));

    // A closure that captures anything CANNOT be coerced.
    let offset = 5;
    let add_offset = |x: i32| x + offset;
    // println!("{}", apply_fn_ptr(add_offset, 3)); // ERROR: different type

    println!("\nAll three closure traits demonstrated.");
}
