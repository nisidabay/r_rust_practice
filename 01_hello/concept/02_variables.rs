/*
 * 02_variables.rs — Practical Rust
 *
 * Question: How do I create and change variables?
 *
 * Rules:
 *   let name = value;       — immutable binding (default)
 *   let mut name = value;   — mutable binding (explicit opt-in)
 *   const NAME: type = val; — compile-time constant (ALWAYS uppercase)
 *
 * Shadowing: reuse a variable name (can change type!)
 */

const GREETING: &str = "Hello from const";

fn main() {
    // Immutable — cannot change
    let x = 5;
    println!("x = {}", x);
    // x = 6;  // COMPILE ERROR: cannot assign twice to immutable variable

    // Mutable — explicit opt-in with 'mut'
    let mut y = 10;
    println!("y (initial) = {}", y);
    y = 20;
    println!("y (mutated) = {}", y);

    // Const — compile-time, always uppercase, ALWAYS annotate type
    println!("Const: {}", GREETING);

    // Shadowing — reuse the same name (creates a NEW binding)
    let z = "hello";
    println!("z as string: {}", z);

    let z = z.len();  // shadow: String → usize (type changed!)
    println!("z as length: {}", z);

    // Shadowing with mut — often cleaner than separate names
    let mut input = String::from("  hello  ");
    input = input.trim().to_string();
    println!("trimmed: {}", input);

    // Why shadow > separate names
    let name        = String::from("Alice");
    let name_len    = name.len();
    let name_upper  = name.to_uppercase();
    println!("{}: {} chars, uppercased: {}", name, name_len, name_upper);

    // With shadowing, same result, fewer intermediate names
    let n = String::from("Bob");
    let n = n.len();
    let n = n as f64 * 2.5;
    println!("n (shadower chain) = {}", n);
}
