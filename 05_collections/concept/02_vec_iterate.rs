// 02_vec_iterate.rs — for, iter(), enumerate(), into_iter() on Vec
//
// Three ways to walk through a Vec and when each matters.
// First 5 lines: loop, borrow, consume, index, enumerate.

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    // 1. for-in — iterates by reference (into_iter desugaring)
    for n in &numbers {
        print!("{n} ");
    }
    println!("(borrowed — numbers still usable)");

    // 2. .iter() — explicit borrow, same effect
    for n in numbers.iter() {
        print!("{n} ");
    }
    println!();

    // 3. .iter_mut() — mutable borrow, can modify in place
    let mut scores = vec![1, 2, 3];
    for s in scores.iter_mut() {
        *s *= 10;
    }
    println!("mutated: {scores:?}");

    // 4. into_iter() — consumes the Vec, takes ownership
    let words = vec!["hi", "bye"];
    for w in words.into_iter() {
        print!("{w} ");
    }
    // println!("{words:?}"); // ERROR: words moved
    println!("(consumed — words no longer available)");

    // 5. enumerate — get index and value
    let colors = vec!["red", "green", "blue"];
    for (i, color) in colors.iter().enumerate() {
        println!("colors[{i}] = {color}");
    }

    // 6. for with range indexing
    for i in 0..colors.len() {
        println!("colors[{i}] = {}", colors[i]);
    }

    // 7. while-let with pop
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("popped {top}");
    }
}
