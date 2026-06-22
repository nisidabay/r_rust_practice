/*
 * ex01_fahrenheit.rs — Exercise 1
 *
 * Task: Read a temperature in Celsius, print it in Fahrenheit.
 *
 * Formula: F = C × 9/5 + 32
 *
 * Run: echo "25" | ./ex01_fahrenheit
 * Expected: 25°C = 77.0°F
 */

use std::io::{self, Write};

fn main() {
    print!("Enter temperature in °C: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let celsius: f64 = input.trim().parse().expect("Not a number");
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;

    println!("{}°C = {:.1}°F", celsius, fahrenheit);
}
