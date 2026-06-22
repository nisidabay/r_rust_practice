// 05_impl_trait.rs — impl Trait syntax (parameter and return types)
//
// impl Trait lets you express "some type that implements this trait"
// without naming the concrete type. Useful for:
//   1. Anonymous generic parameters
//   2. Opaque return types (hide the concrete type)

use std::fmt::Display;

// --- impl Trait as a parameter ---
// Instead of: fn print_it<T: Display>(item: T)
fn print_it(item: impl Display) {
    println!("{}", item);
}

// --- Multiple impl Trait parameters ---
fn print_pair(a: impl Display, b: impl Display) {
    println!("a = {}, b = {}", a, b);
}

// --- impl Trait as a return type ---
// Returns *some type* that implements Display
// The caller cannot name the concrete type
fn make_greeter() -> impl Display {
    "Hello, world!"
}

// --- Real-world: returning a closure ---
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// --- impl Trait vs generics: key difference ---
// Generic: each call can use a DIFFERENT concrete type
fn generic_print<T: Display>(a: T, b: T) {
    println!("{} {}", a, b);
    // a and b must be THE SAME type
}

// impl Trait: each parameter can be a DIFFERENT type
fn impl_print(a: impl Display, b: impl Display) {
    println!("{} {}", a, b);
    // a and b can be DIFFERENT types (both impl Display)
}

fn main() {
    // impl Trait parameter
    print_it(42);
    print_it("hello");
    print_it(3.14);

    // impl Trait pair (different types work!)
    print_pair(42, "hello");

    // impl Trait return type
    let msg = make_greeter();
    println!("Greeter: {}", msg);

    // Returning closures
    let add5 = make_adder(5);
    println!("5 + 3 = {}", add5(3));

    // Generic vs impl Trait difference
    // generic_print(42, "hello");  // ERROR: different types
    generic_print(42, 99);           // OK: same type
    impl_print(42, "hello");         // OK: different types
}
