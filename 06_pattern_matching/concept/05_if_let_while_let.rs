// 05_if_let_while_let.rs — if let, while let, let else patterns
// Learn: when you only care about ONE variant, if let/while let/let else
//        are shorter than a full match.

fn main() {
    // --- if let: match one pattern, ignore the rest ---
    let val: Option<i32> = Some(42);

    // Instead of:
    //   match val { Some(v) => println!("{v}"), _ => {} }
    // Use:
    if let Some(v) = val {
        println!("if let got: {v}");
    }

    // --- if let with else for the other variant ---
    let err_val: Result<i32, &str> = Err("disk full");
    if let Ok(n) = err_val {
        println!("got Ok({n})");
    } else {
        println!("got error, handled in else block");
    }

    // --- while let: loop while a pattern matches ---
    let mut stack = vec![1, 2, 3, 4, 5];

    // pop() returns Option<T>. while let keeps going until None.
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!("<-- popped all");
}

// Proper let-else outside main (let-else requires diverging else branch)
fn div_safe(a: f64, b: f64) -> Option<f64> {
    // let-else: bind the success case, return early on failure
    // Checks that b is nonzero, binds it to nonzero, else returns None
    let Some(nonzero) = (b != 0.0).then_some(b) else {
        return None; // diverges from the match arm — returns from the fn
    };
    Some(a / nonzero)
}
