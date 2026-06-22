/*
 * 05_match.rs — Practical Rust
 *
 * Question: How do I use match in Rust?
 *
 * match = powerful switch statement (it's an expression too!)
 * Arms: pattern => result
 * Must be exhaustive — every possible value must be handled
 * _ is the catch-all (wildcard) pattern
 */

fn main() {
    // Match on integers
    let n = 3;
    let description = match n {
        1 => "one",
        2 => "two",
        3 => "three", // this matches
        _ => "other", // catch-all for anything else
    };
    println!("{} is {}", n, description);

    // Match as expression — assign result
    let x = 7;
    let category = match x {
        1..=3 => "small",    // range pattern
        4..=6 => "medium",
        7..=9 => "large",    // this matches
        10 => "exactly ten",
        _ => "out of range",
    };
    println!("{} is {}", x, category);

    // Match on chars
    let grade = 'B';
    let passing = match grade {
        'A' | 'B' | 'C' => true,  // | means OR
        'D' | 'F' => false,
        _ => {
            println!("Invalid grade: {}", grade);
            false
        }
    };
    println!("Grade {} passing: {}", grade, passing);

    // Match with multiple values (tuple)
    let pair = (1, 5);
    match pair {
        (0, y) => println!("x=0, y={}", y),
        (x, 0) => println!("x={}, y=0", x),
        (x, y) => println!("x={}, y={}", x, y), // catches everything else
    }

    // Match with guard (additional condition)
    let num = 15;
    match num {
        n if n < 0 => println!("negative"),
        n if n % 2 == 0 => println!("even"),
        _ => println!("odd positive"), // 15 → odd positive
    }

    // Match with Option — very common
    let maybe_number: Option<i32> = Some(42);
    match maybe_number {
        Some(val) => println!("got value: {}", val),
        None => println!("got nothing"),
    }
}
