fn main() {
    // match on literals, ranges, multiple patterns, wildcards, and guards
    // Pattern matching is Rust's superpower — like C switch but vastly more powerful

    let value = 7;

    // --- Literals ---
    let desc = match value {
        0 => "zero",
        1 => "one",
        2 => "two",
        // _ is the wildcard — matches anything (like C's default)
        _ => "some other value",
    };
    println!("{}: {}", value, desc);

    // --- Ranges (..= is inclusive) ---
    // match can test ranges with a single arm
    let score = 85;
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        0..=59 => "F",
        _ => "invalid",
    };
    println!("{}: {}", score, grade);

    // --- Multiple patterns with | ---
    let color = "red";
    let is_primary = match color {
        "red" | "blue" | "green" | "yellow" => true,
        _ => false,
    };
    println!("{} is primary: {}", color, is_primary);

    // --- Guards with 'if' ---
    // Guards add extra conditions beyond the pattern itself
    let number = 15;
    match number {
        n if n < 0 => println!("negative"),
        n if n % 2 == 0 => println!("even"),
        n if n % 2 != 0 => println!("odd"),
        _ => unreachable!(), // all cases covered
    }

    // --- Combined: range + guard ---
    let age = 21;
    match age {
        0..=17 => println!("Minor"),
        18..=120 if age >= 65 => println!("Senior"),
        18..=120 => println!("Adult"),
        _ => println!("Impossible age"),
    }
}
