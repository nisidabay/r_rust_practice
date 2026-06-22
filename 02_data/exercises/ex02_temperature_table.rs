/*
 * ex02_temperature_table.rs — Exercise 2
 *
 * Task: Print a table of Celsius temperatures from -20°C to 40°C
 *       in steps of 5, with a Fahrenheit column.
 *
 * Run: ./ex02_temperature_table
 * Expected: formatted table with C and F columns
 */

fn main() {
    // Column headers with padding
    println!("{:<8} {}", "°C", "°F");
    println!("{}", "-------- -----");

    // Loop from -20 to 40 inclusive, step 5
    let mut c = -20;
    while c <= 40 {
        let f = c as f64 * 9.0 / 5.0 + 32.0;
        // Left-align Celsius in 8-char field, Fahrenheit right-aligned (implicit)
        println!("{:<8} {:.1}", c, f);
        c += 5;
    }
}
