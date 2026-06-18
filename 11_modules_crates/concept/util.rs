/// Helper functions used by 01_multiple_files.rs

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn farewell(name: &str) -> String {
    format!("Goodbye, {}. Come back soon!", name)
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
