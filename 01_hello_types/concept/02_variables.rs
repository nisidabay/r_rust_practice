// 02_variables — immutability by default, mut, const, shadowing
// Rust variables are immutable by default — deliberate safety choice.
// Use `mut` to allow mutation; `const` for compile-time constants.
// Shadowing lets us reuse a name with a different type or mutability.

// Compile-time constant — must have a type annotation, can't be `mut`.
const MAX_POINTS: u32 = 100_000;

fn main() {
    // Immutable binding — cannot reassign.
    let x = 5;
    println!("x is {x}");

    // Mutable binding — value can change.
    let mut y = 10;
    println!("y was {y}");
    y += 5;
    println!("y is now {y}");

    // Shadowing — re-declare `x` with a new value (and optionally a new type).
    // The original binding is dead after this line.
    let x = x + 1; // x was 5, now 6
    println!("shadowed x = {x}");

    let x = "now a string!"; // same name, different type
    println!("shadowed x is now: {x}");

    // Constants are inlined at compile time.
    println!("MAX_POINTS = {MAX_POINTS}");

    // Demonstrate mut vs shadowing:
    //   mut changes the value in place (same type).
    //   shadowing creates a new binding (can change type).
}
