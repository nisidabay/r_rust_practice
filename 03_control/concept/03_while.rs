/*
 * 03_while.rs — Practical Rust
 *
 * Question: How do I use while loops in Rust?
 *
 * while condition { ... } — runs as long as condition is true
 * while let pattern = expr { ... } — runs while pattern matches (destructuring)
 */

fn main() {
    // Basic while loop
    let mut n = 5;
    while n > 0 {
        println!("n = {}", n);
        n -= 1;
    }
    println!("liftoff!");

    // while with iterator (manual)
    let mut i = 0;
    let limit = 5;
    while i < limit {
        print!("{} ", i);
        i += 1;
    }
    println!("");

    // while let — destructure Option or Result in a loop
    let mut numbers = vec![10, 20, 30, 40, 50].into_iter();

    // pop() returns Option<T>, while let extracts the value
    while let Some(val) = numbers.next() {
        println!("got: {}", val);
    }

    // while let with a vector (pop from back)
    let mut stack = vec![1, 2, 3, 4, 5];
    while let Some(top) = stack.pop() {
        println!("popped: {}", top);
    }

    // while let with a custom counter (Option-like pattern)
    let mut count = Some(3);
    while let Some(x) = count {
        println!("count is {}", x);
        if x > 1 {
            count = Some(x - 1);
        } else {
            count = None; // next iteration won't match
        }
    }
    println!("done");
}
