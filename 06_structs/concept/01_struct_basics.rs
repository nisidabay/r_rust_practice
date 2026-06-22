/*
 * 01_struct_basics.rs — Practical Rust
 *
 * Question: How do I define my own data type?
 *
 * A struct groups related data together.
 * Define with `struct Name { field: Type, ... }`
 * Create instances with `Name { field: value, ... }`
 * Access fields with `.` (dot notation)
 * Mutate requires `let mut`
 */

fn main() {
    // Define a struct
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        height_cm: f64,
    }

    // Create an instance
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
        height_cm: 165.0,
    };

    // Access fields
    println!("{} is {} years old", alice.name, alice.age);
    println!("Height: {} cm", alice.height_cm);

    // Mutable instance — can change fields
    let mut bob = Person {
        name: String::from("Bob"),
        age: 25,
        height_cm: 180.0,
    };

    bob.age = 26; // mutate
    bob.height_cm = 181.5;
    println!("Bob is now {} and {} cm", bob.age, bob.height_cm);

    // Struct as function parameter
    fn print_person(p: &Person) {
        println!("{} ({} years, {:.1} cm)", p.name, p.age, p.height_cm);
    }

    print_person(&alice);
    print_person(&bob);

    // Debug print (needs #[derive(Debug)])
    println!("Alice: {:?}", alice);
    println!("Bob: {:#?}", bob); // pretty-print
}
