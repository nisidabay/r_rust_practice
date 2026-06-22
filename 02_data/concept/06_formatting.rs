/*
 * 06_formatting.rs — Practical Rust
 *
 * Question: How do I format output precisely?
 *
 * println!("{:b}", x) — binary
 * println!("{:x}", x) — hex lowercase, {:X} for uppercase
 * println!("{:o}", x) — octal
 * println!("{:e}", x) — scientific notation
 * println!("{:?}", x) — debug
 * println!("{:#?}", x) — pretty-print debug
 * println!("{:10}", x) — pad to width 10
 * println!("{:>10}", x) — right-align (default)
 * println!("{:<10}", x) — left-align
 * println!("{:^10}", x) — center
 * println!("{:.2}", x) — precision (2 decimal places for float)
 */

fn main() {
    let n = 255;
    let pi = std::f64::consts::PI;
    let data = vec![1, 2, 3];

    // === Number format specifiers ===
    println!("Decimal:  {}", n);
    println!("Binary:   {:b}", n);   // 11111111
    println!("Hex:      {:x}", n);   // ff
    println!("Hex caps: {:X}", n);   // FF
    println!("Octal:    {:o}", n);   // 377

    // === Scientific notation ===
    println!("Sci:      {:e}", pi);  // 3.1415927e0
    println!("Sci caps: {:E}", pi);  // 3.1415927E0

    // === Debug formatting ===
    println!("Debug:    {:?}", data);   // [1, 2, 3]
    println!("Pretty:   {:#?}", data);  // multi-line pretty

    // === Padding and alignment ===
    println!("|{:10}|", "hello");   // right-aligned in 10 chars (default)
    println!("|{:<10}|", "hello");  // left-aligned
    println!("|{:>10}|", "hello");  // right-aligned
    println!("|{:^10}|", "hello");  // center-aligned
    println!("|{:_<10}|", "hello"); // left-aligned, pad with underscore

    // === Numeric width + padding ===
    println!("|{:10}|", 42);        // right-aligned number
    println!("|{:<10}|", 42);       // left-aligned number
    println!("|{:010}|", 42);       // zero-padded to width 10

    // === Precision for floats ===
    println!("PI = {:.2}", pi);     // 3.14
    println!("PI = {:.4}", pi);     // 3.1416
    println!("PI = {:.10}", pi);    // 3.1415926536

    // === Width + precision combined ===
    println!("|{:10.2}|", pi);      // width 10, precision 2 → "      3.14"
    println!("|{:<10.2}|", pi);     // left-aligned → "3.14      "

    // === Named arguments ===
    println!("{name} is {age} years old", name = "Alice", age = 30);

    // === format! macro (returns a String) ===
    let formatted = format!("{:b} = {}", 255, "binary");
    println!("format! result: {}", formatted);
}
