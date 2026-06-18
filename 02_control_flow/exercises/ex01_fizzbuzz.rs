// Classic FizzBuzz implemented with match on a tuple.
// For each number from 1 to 100, print FizzBuzz, Fizz, Buzz, or the number.
// We use match on (n % 3, n % 5) to express the three conditions cleanly.

fn main() {
    // Iterate 1..=100 and use match on (mod3, mod5) tuple.
    for n in 1..=100 {
        // Pattern-match on the pair of remainders.
        match (n % 3, n % 5) {
            // Divisible by both 3 and 5 (remainders both 0).
            (0, 0) => println!("FizzBuzz"),
            // Divisible by 3 only.
            (0, _) => println!("Fizz"),
            // Divisible by 5 only.
            (_, 0) => println!("Buzz"),
            // Neither — print the number itself.
            _ => println!("{n}"),
        }
    }
}
