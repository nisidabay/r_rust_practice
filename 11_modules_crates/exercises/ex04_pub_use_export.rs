/// BONUS Exercise 4: Design a public API with `pub use` restructuring
///
/// Your task:
/// 1. Create nested modules that define types:
///    - `models::user::User` (fields: name, email)
///    - `models::post::Post` (fields: title, body)
///    - `services::auth::AuthService` (method: login)
///    - `services::post::PostService` (method: create_post)
/// 2. Use `pub use` at the top level to flatten the API so users only write:
///    - `User`, `Post` at the crate root
///    - `AuthService`, `PostService` at the crate root
/// 3. All constructors should accept String or &str as needed.
/// 4. From `main()`, demonstrate constructing each type and calling methods.
///
/// Run: rustc --edition 2021 ex04_pub_use_export.rs && ./ex04_pub_use_export

// TODO: Define the module tree and pub use re-exports

fn main() {
    // TODO: Demonstrate the flattened API
    println!("Replace this with your solution!");
}
