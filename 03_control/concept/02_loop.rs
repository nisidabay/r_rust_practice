/*
 * 02_loop.rs — Practical Rust
 *
 * Question: How do I use loop in Rust?
 *
 * loop = infinite loop (must use break to exit)
 * break can return a VALUE from the loop
 * continue skips to next iteration
 */

fn main() {
    // Basic infinite loop with break
    let mut count = 0;
    loop {
        println!("count = {}", count);
        count += 1;
        if count >= 3 {
            break; // exit the loop
        }
    }

    // loop with break returning a value
    let mut n = 1;
    let result = loop {
        n *= 2;
        if n > 100 {
            break n; // returns n from the loop
        }
    };
    println!("First power of 2 over 100: {}", result);

    // continue — skip rest of iteration
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i == 3 {
            println!("skipping {}", i);
            continue; // skip to next iteration
        }
        println!("i = {}", i);
    }

    // Nested loops with labels
    let mut outer = 0;
    'outer: loop {
        outer += 1;
        let mut inner = 0;
        loop {
            inner += 1;
            println!("outer={} inner={}", outer, inner);
            if inner >= 2 {
                break; // breaks inner loop only
            }
            if outer >= 2 {
                break 'outer; // breaks outer loop
            }
        }
    }
    println!("done with outer={}", outer);
}
