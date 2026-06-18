/// String utility functions (nested inside the `utils` module tree)

pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}
