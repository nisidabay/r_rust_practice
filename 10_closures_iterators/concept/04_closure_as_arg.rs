// 04_closure_as_arg — Passing closures to functions, generic closure params
//
// Closures are often passed to functions to customize behavior.
// We use generic parameters bounded by the closure traits (Fn, FnMut, FnOnce)
// to accept closures as arguments. This is the foundation of iterator adapters.

fn main() {
    // --- 1. Basic: passing a closure to a function ---

    // A simple function that applies a closure to a value
    fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
        f(x)
    }

    let result = apply(|x| x * x, 5);
    println!("apply(|x| x*x, 5) = {result}");

    // --- 2. Multiple closure parameters ---
    fn transform<F, G>(x: i32, f: F, g: G) -> i32
    where
        F: Fn(i32) -> i32,
        G: Fn(i32) -> i32,
    {
        g(f(x))
    }

    let r = transform(10, |x| x + 1, |x| x * 2);
    println!("transform(10, +1, *2) = {r}");

    // --- 3. Closure that returns a bool (predicate) ---
    fn check<F: Fn(i32) -> bool>(val: i32, predicate: F) -> bool {
        predicate(val)
    }

    let is_positive = |x| x > 0;
    println!("check(5, is_positive) = {}", check(5, is_positive));
    println!("check(-3, is_positive) = {}", check(-3, is_positive));

    // --- 4. FnMut closure parameter (stateful closure) ---
    fn apply_mut<F: FnMut(i32) -> i32>(mut f: F, x: i32) -> i32 {
        f(x)
    }

    let mut running = 0;
    let mut accum = |val: i32| {
        running += val;
        running
    };
    println!("apply_mut(accum, 5) = {}", apply_mut(&mut accum, 5));
    println!("apply_mut(accum, 3) = {}", apply_mut(&mut accum, 3));

    // --- 5. FnOnce closure parameter ---
    fn consume<F: FnOnce(String) -> String>(f: F, s: String) -> String {
        f(s) // can only be called once
    }

    let greeting = |name: String| format!("Hello, {name}!");
    let msg = consume(greeting, "Rust".to_string());
    println!("{msg}");

    // --- 6. Returning a closure from a function ---
    fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor // captures factor by value (move)
    }

    let doubler = make_multiplier(2);
    let tripler = make_multiplier(3);
    println!("doubler(7) = {}, tripler(7) = {}", doubler(7), tripler(7));

    // --- 7. Practical: customizing a computation ---
    fn process_vec(v: &[i32], mut transform: impl FnMut(&i32) -> i32) -> Vec<i32> {
        v.iter().map(|x| transform(x)).collect()
    }

    let nums = vec![1, 2, 3, 4, 5];
    let doubled = process_vec(&nums, |x| x * 2);
    let squared = process_vec(&nums, |x| x * x);
    println!("Doubled: {:?}", doubled);
    println!("Squared: {:?}", squared);

    println!("\nAll closure-as-argument patterns demonstrated.");
}
