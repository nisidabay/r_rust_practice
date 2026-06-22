// 03_derive.rs — #[derive(Debug, Clone, PartialEq)]
//
// Rust can auto-implement certain common traits via #[derive].
// The compiler generates the trait implementation for you.

// --- Debug ---
// Enables {:?} and {:#?} formatting for debugging output
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Without Debug, this would fail:
// println!("{:?}", point);

// --- Clone ---
// Enables .clone() to make a deep copy
#[derive(Clone)]
struct Config {
    name: String,
    version: u32,
}

// Without Clone, you'd need to manually copy each field

// --- PartialEq ---
// Enables == and != comparisons
#[derive(PartialEq)]
enum Status {
    Active,
    Inactive,
    Pending,
}

// --- Combine them ---
#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u64,
    name: String,
    active: bool,
}

fn main() {
    // Debug
    let p = Point { x: 10, y: 20 };
    println!("Debug:     {:?}", p);
    println!("Pretty:    {:#?}", p);

    // Clone
    let c1 = Config {
        name: "app".to_string(),
        version: 1,
    };
    let c2 = c1.clone();  // Deep copy
    println!("Cloned:    {} v{}", c2.name, c2.version);

    // PartialEq
    println!("Equal?:    {}", Status::Active == Status::Active);
    println!("Not eq?:   {}", Status::Active == Status::Inactive);

    // All together
    let u1 = User {
        id: 1,
        name: "Alice".to_string(),
        active: true,
    };
    let u2 = u1.clone();
    println!("User:      {:?}", u1);
    println!("Clone eq?: {}", u1 == u2);

    println!("\n--- What each derive does ---");
    println!("Debug:     {{:?}} formatting (required by almost everything)");
    println!("Clone:     .clone() for deep copies (heap data needs this)");
    println!("PartialEq: == and != comparisons");
    println!("Copy:      Implicit copy on assignment (only for stack-only types)");
    println!("Eq:        Strict equality (needed for HashMap keys with PartialEq)");
    println!("Hash:      .hash() for use in HashMap/HashSet keys");
    println!("Default:   .default() to create a zero-value instance");
}
