// ex01_parse_number — Parse a string to i32, return descriptive error
//
// Problem: Write a function `parse_number(s: &str) -> Result<i32, String>`
// that parses a string into an i32. On failure, return a descriptive error
// message that includes the original input.
//
// Then use it to parse these inputs and print results:
//   "42", "-5", "3.14", "abc", "", "9999999999" (overflows i32)
//
// Hint: Use `.parse::<i32>()` and `.map_err()` to convert the error.

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("cannot parse \"{s}\" as i32: {e}"))
}

fn main() {
    let inputs = ["42", "-5", "3.14", "abc", "", "9999999999"];

    for input in &inputs {
        match parse_number(input) {
            Ok(n) => println!("OK:   \"{input}\" -> {n}"),
            Err(e) => println!("ERR:  \"{input}\" -> {e}"),
        }
    }
}
