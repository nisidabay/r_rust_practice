/*
 * 02_booleans.rs — Practical Rust
 *
 * Question: How do I use booleans in Rust?
 *
 * bool: true or false (1 byte)
 * Comparison operators: == != < > <= >=
 * Logical: && (AND), || (OR), ! (NOT)
 * Short-circuit: && and || stop evaluating early
 */

fn main() {
    // bool literals
    let t: bool = true;
    let f: bool = false;
    println!("true = {}, false = {}", t, f);

    // Comparison operators produce bool
    let x = 10;
    let y = 20;
    println!("{} < {} = {}", x, y, x < y);
    println!("{} == {} = {}", x, y, x == y);
    println!("{} != {} = {}", x, y, x != y);

    // Logical AND && — both must be true
    println!("true && true = {}", true && true);
    println!("true && false = {}", true && false);

    // Logical OR || — at least one must be true
    println!("true || false = {}", true || false);
    println!("false || false = {}", false || false);

    // Logical NOT ! — inverts
    println!("!true = {}", !true);
    println!("!false = {}", !false);

    // Short-circuit: && stops at first false
    let short_circuit = false && { println!("this never prints"); true };
    println!("short-circuit &&: {}", short_circuit);

    // Short-circuit: || stops at first true
    let short_circuit2 = true || { println!("this never prints either"); false };
    println!("short-circuit ||: {}", short_circuit2);

    // Using bool in expressions
    let age = 25;
    let can_drive = age >= 16;
    let can_vote = age >= 18;
    println!("Age {}: can_drive={}, can_vote={}", age, can_drive, can_vote);

    // ! takes precedence over &&, which takes precedence over ||
    // Use parentheses to be clear
    let result = (10 > 5) && (3 < 7) || !false;
    println!("complex expression: {}", result);
}
