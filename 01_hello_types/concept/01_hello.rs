// 01_hello — minimal Rust program
// `fn main()` is the program entry point.
// `println!` is a macro that prints text with a newline.
// The `!` marks it as a macro, not a regular function.

fn main() {
    // Print a string literal to stdout. Rust infers no types here.
    println!("Hello, world!");

    // We can also print raw string slices directly.
    println!("Hello, 또 다른 세상!");

    // println! uses a format string — arguments replace {} placeholders.
    let name = "Hermes";
    println!("Hello, {}!", name);
}
