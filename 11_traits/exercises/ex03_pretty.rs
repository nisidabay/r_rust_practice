// ex03_pretty.rs — Trait PrettyPrint { fn pretty(&self) -> String }
//
// Impl for Vec<T: Display> and Option<T: Display>
//
// Usage: ./ex03_pretty
// Expected: pretty-printed vecs and options

use std::fmt::Display;

trait PrettyPrint {
    fn pretty(&self) -> String;
}

impl<T: Display> PrettyPrint for Vec<T> {
    fn pretty(&self) -> String {
        if self.is_empty() {
            return "[]".to_string();
        }
        let items: Vec<String> = self.iter().map(|item| format!("{}", item)).collect();
        format!("[{}]", items.join(", "))
    }
}

impl<T: Display> PrettyPrint for Option<T> {
    fn pretty(&self) -> String {
        match self {
            Some(val) => format!("Some({})", val),
            None => "None".to_string(),
        }
    }
}

fn main() {
    // Vec pretty printing
    let empty: Vec<i32> = vec![];
    let numbers = vec![1, 2, 3, 4, 5];
    let words = vec!["hello", "world", "rust"];
    let mixed_i32 = vec![10, -5, 1000];

    println!("Vec pretty printing:");
    println!("  empty:  {}", empty.pretty());
    println!("  nums:   {}", numbers.pretty());
    println!("  words:  {}", words.pretty());
    println!("  mixed:  {}", mixed_i32.pretty());

    // Option pretty printing
    let some_num: Option<i32> = Some(42);
    let none_num: Option<i32> = None;
    let some_str: Option<&str> = Some("hello");
    let some_big: Option<i32> = Some(999999);

    println!("\nOption pretty printing:");
    println!("  Some(42):     {}", some_num.pretty());
    println!("  None:         {}", none_num.pretty());
    println!("  Some(hello):  {}", some_str.pretty());
    println!("  Some(999999): {}", some_big.pretty());
}
