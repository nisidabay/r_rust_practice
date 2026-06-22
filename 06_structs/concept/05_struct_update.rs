/*
 * 05_struct_update.rs — Practical Rust
 *
 * Question: How do I create a struct from another with some changes?
 *
 * Struct update syntax: Struct { field: new_val, ..other }
 * This creates a NEW struct using all fields from `other`, then
 * overrides the specified fields.
 * Destructuring: let Struct { field1, field2 } = instance;
 */

fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        city: String,
        occupation: String,
    }

    let alice = Person {
        name: String::from("Alice"),
        age: 30,
        city: String::from("New York"),
        occupation: String::from("Engineer"),
    };

    // Struct update — create new Person from alice, overriding age
    let older_alice = Person {
        age: 31,
        ..alice // must come last — fills remaining fields
    };

    // Note: alice.name, alice.city, alice.occupation MOVED into older_alice
    // alice is partially invalid now (some fields moved)
    // println!("alice: {:?}", alice); // COMPILE ERROR
    println!("older_alice: {:?}", older_alice);

    // To keep both, clone first:
    let bob = Person {
        name: String::from("Bob"),
        age: 25,
        city: String::from("Boston"),
        occupation: String::from("Designer"),
    };

    // With Copy types only, the original stays valid
    #[derive(Debug, Clone)]
    struct Config {
        version: u32,
        debug: bool,
        max_connections: u32,
    }

    let default_config = Config {
        version: 1,
        debug: false,
        max_connections: 100,
    };

    // All fields are Copy (u32, bool) — original stays valid
    let dev_config = Config {
        debug: true,
        ..default_config
    };
    println!("default: {:?}", default_config); // still valid!
    println!("dev:     {:?}", dev_config);

    // Destructuring
    let Config { version, debug, max_connections } = dev_config;
    println!("Version: {}, Debug: {}, Max: {}", version, debug, max_connections);
}
