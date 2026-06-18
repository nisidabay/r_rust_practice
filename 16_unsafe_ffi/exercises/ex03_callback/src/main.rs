// ex03: Unsafe extern "C" callback function pattern
//
// Many C libraries use callbacks — function pointers passed to C code
// that C calls back later. In Rust, this requires `extern "C" fn`
// function pointers and careful lifetime management.
//
// Your task: implement a simple callback registry using extern "C" fns.

use std::ffi::CString;

/// The callback type — a C-compatible function pointer.
/// Takes a C string pointer (data) and returns void.
type Callback = extern "C" fn(*const i8);

// --- Two callback implementations ---

/// A callback that prints "Hello" prefix
extern "C" fn hello_callback(name: *const i8) {
    let c_str = unsafe { std::ffi::CStr::from_ptr(name) };
    if let Ok(s) = c_str.to_str() {
        println!("Hello, {}!", s);
    }
}

/// A callback that prints "Goodbye" prefix
extern "C" fn goodbye_callback(name: *const i8) {
    let c_str = unsafe { std::ffi::CStr::from_ptr(name) };
    if let Ok(s) = c_str.to_str() {
        println!("Goodbye, {}!", s);
    }
}

/// A callback that counts uppercase letters
extern "C" fn count_uppercase_callback(name: *const i8) {
    let c_str = unsafe { std::ffi::CStr::from_ptr(name) };
    if let Ok(s) = c_str.to_str() {
        let count = s.chars().filter(|c| c.is_uppercase()).count();
        println!(
            "\"{}\" has {} uppercase letter(s)",
            s, count
        );
    }
}

// --- Callback registry (simulating C library) ---

/// Simulates a C library that stores and invokes a callback.
/// In real FFI, this would be a C library's callback registration.
struct CallbackRegistry {
    cb: Option<Callback>,
}

impl CallbackRegistry {
    const fn new() -> Self {
        CallbackRegistry { cb: None }
    }

    /// Register a callback (like C's `register_callback`)
    fn register(&mut self, cb: Callback) {
        self.cb = Some(cb);
        println!("Callback registered");
    }

    /// Invoke the registered callback (like C calling back into Rust)
    fn invoke(&self, name: &str) {
        if let Some(cb) = self.cb {
            let c_name = CString::new(name).expect("CString failed");
            cb(c_name.as_ptr());
        } else {
            println!("No callback registered");
        }
    }
}

fn main() {
    println!("=== Exercise 03: Extern C Callback Pattern ===");

    let mut registry = CallbackRegistry::new();
    registry.invoke("Nobody"); // no callback yet

    registry.register(hello_callback);
    registry.invoke("Alice");
    registry.invoke("Bob");

    registry.register(goodbye_callback);
    registry.invoke("Charlie");

    registry.register(count_uppercase_callback);
    registry.invoke("RustLangIsGreat");

    // --- Function pointer table ---
    println!();
    println!("--- Dispatching multiple callbacks ---");

    let callbacks: &[Callback] = &[
        hello_callback,
        count_uppercase_callback,
        goodbye_callback,
    ];

    for (i, cb) in callbacks.iter().enumerate() {
        let name = CString::new("CallbackUser").unwrap();
        print!("Callback {}: ", i);
        cb(name.as_ptr());
    }

    println!();
    println!("Key points:");
    println!("  - `extern \"C\" fn(...)` types are C-compatible function pointers");
    println!("  - The callback must be `#[no_mangle]` or exported when linking with C");
    println!("  - Passing Rust closures to C is tricky — use extern fn or unsafe trampolines");
    println!("  - Always handle null pointers and validate C strings in callbacks");
}
