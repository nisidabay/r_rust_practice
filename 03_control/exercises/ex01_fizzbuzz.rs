/*
 * ex01_fizzbuzz.rs — Exercise 1
 *
 * Task: Print numbers 1..100.
 *       If divisible by 3 → "Fizz"
 *       If divisible by 5 → "Buzz"
 *       If divisible by both → "FizzBuzz"
 *
 * Run: ./ex01_fizzbuzz
 */

fn main() {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }
}
