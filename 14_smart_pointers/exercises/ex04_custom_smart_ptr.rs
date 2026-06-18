// ex04_custom_smart_ptr.rs (BONUS) — Implement a simple MyBox<T> with Deref and Drop
//
// Build your own smart pointer from scratch. MyBox<T> will:
// - Store a value on the heap using Box
// - Implement Deref so you can use * and method calls
// - Implement Drop that logs when cleanup happens
// - Track how many times deref is called (for debugging)

use std::ops::{Deref, Drop};

// TODO: Define MyBox<T> struct with a Box<T> inside
// struct MyBox<T>(Box<T>);

// TODO: Implement MyBox::new(value) -> MyBox<T>

// TODO: Implement Deref<Target = T> for MyBox<T>

// TODO: Implement Drop for MyBox<T> — print "MyBox dropped!"

fn main() {
    // Test basic creation and dereference
    let x = MyBox::new(42);
    assert_eq!(*x, 42);
    println!("Basic deref: {} (OK)", *x);

    // Test deref coercion (MyBox<i32> -> &i32)
    fn takes_ref(y: &i32) -> i32 {
        *y + 1
    }
    assert_eq!(takes_ref(&x), 43);
    println!("Deref coercion: takes_ref(&x) = {} (OK)", takes_ref(&x));

    // Test with String (deref chaining: MyBox<String> -> &String -> &str)
    let s = MyBox::new(String::from("hello"));
    assert_eq!(s.len(), 5); // Deref to String, then String::Deref to str
    assert_eq!(&s[..], "hello");
    println!("String deref chain: len={}, content='{}' (OK)", s.len(), &s[..]);

    // Test Drop — the value should be dropped when it goes out of scope
    {
        let inner = MyBox::new("scoped");
        println!("  Inside inner scope, value = {}", *inner);
    }
    println!("  (inner should have been dropped by now)");

    // Test multiple operations
    let mut val = 0i32;
    for i in 0..3 {
        let boxed = MyBox::new(i);
        val += *boxed;
    }
    assert_eq!(val, 0 + 1 + 2);
    println!("Loop sum: {} (OK)", val);

    println!("\nAll BONUS tests passed! 🎉");
}
