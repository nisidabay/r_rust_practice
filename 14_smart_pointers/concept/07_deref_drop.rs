// 07_deref_drop.rs — Deref and Drop traits, how smart pointers work under the hood
//
// Deref and Drop are the two traits that make smart pointers "smart."
// - Deref: allows the * operator and implicit deref coercion
// - Drop: runs cleanup when the value goes out of scope
//
// Every smart pointer (Box, Rc, RefCell, etc.) implements these traits
// to provide custom ownership semantics.

use std::ops::{Deref, Drop};

// A simple smart pointer that logs its lifecycle.
// This is how Box works under the hood (simplified).
struct MyBox<T> {
    value: T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        println!("  MyBox::new — allocating value");
        MyBox { value }
    }
}

// Deref<Target = T> lets Rust treat &MyBox<T> as &T automatically.
// Without this, you'd need (*mybox).method() everywhere.
// With Deref, Rust inserts * automatically via deref coercion.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        println!("  MyBox::deref — returning reference to inner value");
        &self.value
    }
}

// Drop runs when the value goes out of scope.
// For Box, this frees the heap memory.
// For Rc, this decrements the reference count (and frees if zero).
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("  MyBox::drop — cleaning up");
        // In a real Box, this would free heap memory.
        // Here the value gets dropped automatically by Rust.
    }
}

fn main() {
    println!("=== MyBox<T> with Deref and Drop ===");

    // Creating our smart pointer
    let x = 42;
    let mybox = MyBox::new(x);

    // Deref coercion: Rust converts &MyBox<i32> to &i32 automatically
    // because MyBox implements Deref<Target = i32>.
    println!("Value via deref coercion: {}", *mybox);

    // Deref actually returns &i32, so this works:
    assert_eq!(*mybox, 42);

    {
        let inner = MyBox::new(String::from("hello"));
        // Deref coercion: &MyBox<String> -> &String -> &str
        // MyBox<String> -> Deref -> String -> Deref -> str
        // So we can call str methods directly!
        println!("String length via deref chain: {}", inner.len());
    } // inner drops here — Drop runs

    println!("--- End of scope for mybox ---");
    // mybox drops here

    println!("\n=== Deref coercion examples ===");

    // Function that takes &str
    fn print_str(s: &str) {
        println!("  print_str received: '{}'", s);
    }

    let name = MyBox::new(String::from("Alice"));
    // Deref coercion chain: &MyBox<String> -> &String -> &str
    // Three steps: 1) &name is &MyBox<String>
    //              2) Deref<Target=String> on MyBox -> &String
    //              3) Deref<Target=str> on String -> &str
    print_str(&name);
    assert_eq!(name.to_uppercase(), "ALICE");

    println!("\n=== Drop order ===");
    drop_demo();

    println!("\n=== Real smart pointers comparison ===");
    // Box: heap allocation + Deref + Drop
    let boxed = Box::new([0u8; 1024]); // 1KB on heap instead of stack
    println!("Box<[u8; 1024]>: stack footprint = {} bytes", std::mem::size_of_val(&boxed));

    // Rc: reference counting + Deref + custom Drop
    let rc1 = std::rc::Rc::new(42);
    let rc2 = std::rc::Rc::clone(&rc1);
    println!("Rc<i32> count: {}", std::rc::Rc::strong_count(&rc1));

    // RefCell: runtime borrow tracking
    let cell = std::cell::RefCell::new(10);
    *cell.borrow_mut() += 5;
    println!("RefCell value after mutation: {}", cell.borrow());
}

/// Demonstrates drop order for nested smart pointers.
fn drop_demo() {
    let outer = MyBox::new("outer");
    {
        let inner = MyBox::new("inner");
        println!("  Inside inner scope");
        // inner.drop() runs here (end of block)
    }
    println!("  Outside inner scope");
    // outer.drop() runs here (end of function scope)
}
