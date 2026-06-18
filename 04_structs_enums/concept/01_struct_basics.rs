// Structs let you bundle related values into a named type.
// Each field has a name and a type — like a lightweight class without methods.

fn main() {
    // --- Defining and instantiating a struct ---
    // Define a struct with named fields.
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // Create an instance by specifying field values.
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("User: {} <{}>", user1.username, user1.email);

    // --- Mutable struct ---
    // The entire instance must be mutable to modify fields.
    let mut user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        sign_in_count: 1,
        active: false,
    };
    user2.active = true;
    println!("{} active: {}", user2.username, user2.active);

    // --- Field init shorthand ---
    // When the field name matches the variable name, you can use shorthand.
    let username = String::from("carol");
    let email = String::from("carol@example.com");
    let user3 = User {
        username,  // shorthand for username: username
        email,     // shorthand for email: email
        sign_in_count: 2,
        active: true,
    };
    println!("{}: {} (sign-ins: {})", user3.username, user3.email, user3.sign_in_count);

    // --- Struct update syntax ---
    // Copy remaining fields from another instance with `..other`.
    let user4 = User {
        username: String::from("dave"),
        email: String::from("dave@example.com"),
        ..user1  // fills sign_in_count and active from user1
        // Note: user1.username and user1.email are MOVED, so user1 is no longer usable.
    };
    println!("{} active: {}", user4.username, user4.active);

    // --- Unit-like struct (no fields) ---
    // Useful for traits or markers.
    struct AlwaysEqual;
    let _marker = AlwaysEqual;
    println!("Unit-like struct created (no fields)");

    // --- Debug printing with #[derive(Debug)] ---
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 10, y: 20 };
    println!("Point: {:?}", p);
    println!("Pretty-point: {:#?}", p);
}
