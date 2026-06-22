// 02_explicit.rs — When elision isn't enough: explicit lifetime annotations
//
// Some functions have multiple input references but one output. The compiler
// can't guess which input the output borrows from, so you must annotate.

// Without annotations: "missing lifetime specifier"
// fn longest(x: &str, y: &str) -> &str { ... }  // ERROR
//
// We tell the compiler: "the output lives as long as both x and y"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Both x and y must outlive 'a. The output borrows from whichever we return.
    if x.len() > y.len() { x } else { y }
}

// The annotation doesn't mean both inputs have the SAME lifetime.
// It means: pick the SHORTER of the two lifetimes — that's 'a.
// The output can't outlive either input.

// Function that chooses the first string — output only tied to x
fn always_first<'a>(x: &'a str, _y: &str) -> &'a str {
    // y has no lifetime constraint — it can be dropped early
    x
}

fn main() {
    // Two strings with potentially different lifetimes
    let s1 = String::from("a long string");
    let result;

    {
        let s2 = String::from("short");
        // result borrows from both s1 and s2 — must not outlive either
        result = longest(&s1, &s2);
        println!("longest in this scope: '{}'", result);
    } // s2 dropped here

    // result can't be used after s2 dies — compiler prevents this!
    // println!("{}", result);  // WOULD NOT COMPILE

    // But always_first only borrows from x, so y can have shorter lifetime
    let x = String::from("stays alive");
    let chosen;
    {
        let y = String::from("dies here");
        chosen = always_first(&x, &y);
    } // y dropped — OK, chosen borrows from x
    println!("always_first chose: '{}'", chosen);

    println!("Explicit lifetimes: compiler now knows which reference lives how long.");
}
