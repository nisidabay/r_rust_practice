// ex03_calc.rs — Simple calculator: parse "3 + 4" with pattern matching
// WHY: Processing "operator operand" strings is a classic use for
//      pattern matching on split strings and parsed numbers.

/// Evaluate a simple expression string like "3 + 4".
///
/// Supported format: "<number> <operator> <number>"
/// Operators: +, -, *, /
/// Returns Some(f64) result, or None if parsing/evaluation fails.
/// Division by zero should also return None.
fn eval(expr: &str) -> Option<f64> {
    // TODO: Parse the expression using pattern matching.
    // Steps:
    //   1. Split the string on whitespace: expr.split_whitespace()
    //   2. Collect into a Vec<&str> or use pattern matching on
    //      the iterator. Hint: match on .collect::<Vec<_>>().as_slice()
    //      with [a, op, b] pattern!
    //   3. Parse a and b with str::parse::<f64>()
    //   4. Match on the operator string to select the operation
    //   5. Return the result (or None if op is unknown or division by zero)
    //
    // Pattern matching ideas:
    //   - match parts.as_slice() { [a, op, b] => ... , _ => None }
    //   - Use if let with pattern matching for parse results
    todo!("implement eval")
}

fn main() {
    let tests = [
        "3 + 4",
        "10 * 2.5",
        "100 / 4",
        "7 - 3",
        "5 / 0",  // division by zero
        "bad input",
        "1 + 2 + 3",  // too many tokens
        "hello + world", // non-numeric
    ];

    println!("=== Simple Calculator ===");
    for expr in &tests {
        match eval(expr) {
            Some(result) => println!("{expr} = {result}"),
            None => println!("{expr} => error"),
        }
    }
}
