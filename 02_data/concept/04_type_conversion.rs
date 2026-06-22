/*
 * 04_type_conversion.rs — Practical Rust
 *
 * Question: How do I convert between types?
 *
 * Three main ways:
 * 1. `as` — primitive-to-primitive casting (cheap, can truncate)
 * 2. From/Into traits — infallible conversions between related types
 * 3. parse() — string → any type that implements FromStr
 */

fn main() {
    // === 1. `as` casting (primitives only) ===

    // Float → integer truncates
    let pi = 3.99;
    let truncated: i32 = pi as i32;
    println!("f64 {} as i32 = {} (truncated, not rounded)", pi, truncated);

    // Integer → float
    let n: i32 = 42;
    let f: f64 = n as f64;
    println!("i32 {} as f64 = {}", n, f);

    // Large → small wraps around
    let big: u16 = 500;
    let small: u8 = big as u8; // 500 - 256 = 244
    println!("u16 {} as u8 = {} (wraps at 256)", big, small);

    // char ↔ integer
    let c = 'A' as u8; // 'A' = 65 in ASCII
    println!("char 'A' as u8 = {}", c);

    // === 2. From and Into traits ===

    // From: you convert INTO the type that implements From
    let s: String = String::from("hello from From trait");
    println!("{}", s);

    // Into: .into() infers target type from context
    let s2: String = "hello from Into trait".into();
    println!("{}", s2);

    // From on numeric types (between compatible types)
    let x: i64 = 42i32.into(); // i32 → i64 is safe
    println!("i32 into i64: {}", x);

    // u32 → u64 via Into
    let small_u: u32 = 100;
    let big_u: u64 = small_u.into();
    println!("u32 {} into u64 = {}", small_u, big_u);

    // === 3. parse() with turbofish ::<> ===

    // Parse a string into a number
    let input = "42";
    let num: i32 = input.parse().unwrap();
    println!("parsed '{}' → i32 {}", input, num);

    // Turbofish notation: ::
    let float_val = "3.14".parse::<f64>().unwrap();
    println!("parsed '3.14' → f64 {}", float_val);

    // Parsing can fail — use unwrap_or for a default
    let failed = "not_a_number".parse::<i32>().unwrap_or(-1);
    println!("failed parse defaults to: {}", failed);

    // Parse into other types too
    let truth = "true".parse::<bool>().unwrap();
    println!("parsed 'true' → bool {}", truth);
}
