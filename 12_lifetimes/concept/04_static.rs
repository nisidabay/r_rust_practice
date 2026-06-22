// 04_static.rs — The 'static lifetime: lives for the entire program
//
// 'static means the reference is valid for the entire program duration.
// It's the longest possible lifetime.

// String literals are &'static str because they're embedded in the binary.
// They live for the entire program — no allocation, no deallocation.
const GREETING: &str = "Hello, world!";  // implicitly &'static str
static GOODBYE: &str = "Goodbye!";       // also 'static

// A function returning &'static str — the string must be hardcoded
fn get_env_var(key: &str) -> &'static str {
    // match arms return string literals which are 'static
    match key {
        "lang" => "Rust",
        "version" => "2021",
        _ => "unknown",
    }
}

// When you actually NEED 'static: the reference must live forever.
// This is rare in practice. Most "I need 'static" cases really mean
// "I need to ensure the data lives long enough" — use owned types instead.

// Common pattern: leaking a Box to get 'static (deliberate memory leak)
fn leak_string(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())  // Box::leak creates a 'static reference
}

fn main() {
    // String literals are 'static — they live in the binary's data section
    let s: &'static str = "I live forever";
    println!("Static literal: {}", s);

    // Constants are also 'static
    println!("Static constant: {}", GREETING);

    // Function returning 'static
    println!("env lang: {}", get_env_var("lang"));

    // Boxing and leaking to get a 'static reference
    let leaked = leak_string(String::from("Leaked memory"));
    println!("Leaked to 'static: {}", leaked);

    // When you DON'T need 'static:
    // If you just need a reference that lives as long as some data,
    // use a generic lifetime, not 'static.
    let local = String::from("local data");
    let r = &local;  // &String, not &'static String — compiler tracks local lifetime
    println!("Local reference (not 'static): {}", r);

    println!("'static: data that lives forever, embedded in the program.");
}
