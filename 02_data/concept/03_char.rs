/*
 * 03_char.rs — Practical Rust
 *
 * Question: How do I work with characters in Rust?
 *
 * char = 4-byte Unicode scalar value (not ASCII!)
 * Use single quotes: 'a', 'π', '🚀'
 * Methods: is_alphabetic(), is_digit(), to_uppercase(), to_lowercase()
 */

fn main() {
    // A char is always 4 bytes and holds any Unicode scalar value
    let letter: char = 'R';
    let greek: char = 'π';
    let rocket: char = '🚀';
    println!("chars: {} {} {}", letter, greek, rocket);

    // char size in bytes (always 4, regardless of the character)
    println!("sizeof(char) = {} bytes", std::mem::size_of::<char>());

    // is_alphabetic() — checks if char is a Unicode letter
    println!("'a'.is_alphabetic() = {}", 'a'.is_alphabetic());
    println!("'π'.is_alphabetic() = {}", 'π'.is_alphabetic());
    println!("'3'.is_alphabetic() = {}", '3'.is_alphabetic());
    println!("'😀'.is_alphabetic() = {}", '😀'.is_alphabetic()); // emoji, not a letter

    // is_digit(radix) — checks if char is a digit in given base
    println!("'5'.is_digit(10) = {}", '5'.is_digit(10));
    println!("'F'.is_digit(16) = {}", 'F'.is_digit(16)); // hex digit!
    println!("'G'.is_digit(16) = {}", 'G'.is_digit(16)); // not in hex

    // to_uppercase() and to_lowercase() — Unicode-aware
    // Returns an iterator because some chars expand (e.g., ß → SS)
    println!("'a'.to_uppercase() = {}", 'a'.to_uppercase().to_string());
    println!("'R'.to_lowercase() = {}", 'R'.to_lowercase().to_string());
    println!("'π'.to_uppercase() = {}", 'π'.to_uppercase().to_string());
    println!("'ß'.to_uppercase() = {}", 'ß'.to_uppercase().to_string()); // ß → SS

    // char from a number (u8)
    let byte: u8 = 65;
    println!("u8 {} as char = '{}'", byte, byte as char);

    // char back to u32
    let c = '🦀';
    println!("char '{}' as u32 = {}", c, c as u32);

    // is_whitespace(), is_numeric(), is_ascii_*() are also available
    println!("' '.is_whitespace() = {}", ' '.is_whitespace());
    println!("'\\t'.is_whitespace() = {}", '\t'.is_whitespace());
}
