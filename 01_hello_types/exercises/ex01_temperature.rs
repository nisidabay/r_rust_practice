// ex01_temperature — Convert Celsius to Fahrenheit and back
// Uses: variables, arithmetic, output formatting.
// The user provides a temperature in Celsius; the program prints
// Fahrenheit and Kelvin equivalents. Run with: ./ex01_temperature <celsius>

use std::env;

fn main() {
    // Grab first CLI argument, default to "0.0" if missing.
    let args: Vec<String> = env::args().collect();
    let celsius_str = if args.len() > 1 { &args[1] } else { "0.0" };

    // Parse the string into f64 — runtime type conversion.
    let celsius: f64 = celsius_str
        .parse()
        .expect("Usage: ex01_temperature <celsius>");

    // Conversions: F = C * 9/5 + 32, K = C + 273.15
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    let kelvin = celsius + 273.15;

    // Print with 2 decimal places using format specifier.
    println!("{celsius:.1}°C = {fahrenheit:.1}°F = {kelvin:.1}K");
}
