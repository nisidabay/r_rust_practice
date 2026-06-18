/// Module loaded from a separate file by 03_modules_file.rs

pub fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn goodbye(name: &str) -> String {
    format!("Goodbye, {}!", name)
}
