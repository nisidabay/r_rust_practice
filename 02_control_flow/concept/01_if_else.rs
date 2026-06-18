// If/else is a fundamental branching construct in Rust.
// It's an expression, not a statement — every branch must return
// the same type, and the result can be assigned to a variable.

fn main() {
    // --- Basic if/else ---
    // The condition must be a `bool`, no implicit numeric coercion.
    let temperature = 22;

    // We use if/else to branch on a boolean condition.
    // Because `if` is an expression, we can assign its result.
    let feeling = if temperature > 25 {
        // Each arm is a block expression; the last expression is returned.
        "hot"
    } else if temperature < 10 {
        "cold"
    } else {
        // `else` is required when assigning; not needed when used as a statement.
        "pleasant"
    };
    // Note the semicolon — `let` is a statement, `if` is the value expression.
    println!("At {temperature}°C it feels {feeling}");

    // --- Boolean expressions in conditions ---
    // Rust uses && (and), || (or), and ! (not), like C-family languages.
    let score = 85;
    let passed = score >= 60;
    let honors = score >= 90;

    // Compound conditions must be explicitly combined.
    if passed && !honors {
        println!("You passed (no honors)");
    } else if passed && honors {
        println!("Honors student!");
    } else {
        println!("Did not pass");
    }

    // --- if in expressions (ternary-style) ---
    // Rust has no ternary operator (`cond ? a : b`); use if/else instead.
    let age = 20;
    let access = if age >= 18 { "allowed" } else { "denied" };
    // Both branches must be the same type; here both are &str.
    println!("Access {access}");

    // --- Early return with if ---
    // We can return early from a function inside an if branch.
    fn check_threshold(value: i32) -> &'static str {
        if value < 0 {
            // Return early with a special value; short-circuits the function.
            return "negative";
        }
        if value == 0 {
            return "zero";
        }
        // Normal fallthrough at the end of the function body.
        "positive"
    }
    println!("check_threshold(-5): {}", check_threshold(-5));
    println!("check_threshold(0):  {}", check_threshold(0));
    println!("check_threshold(42): {}", check_threshold(42));
}
