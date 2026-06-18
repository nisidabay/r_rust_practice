// 08_abi_calling.rs — extern "C" functions, linking to C libraries
//
// What do you do when you need to break the rules?
//
// The `extern "C"` ABI tells the Rust compiler to use the C calling convention
// for a function, making it compatible with C libraries and tools.
//
// You can:
//   1. Declare C functions to call from Rust (extern block)
//   2. Export Rust functions callable from C (extern "C" fn with #[no_mangle])

// --- Exporting a Rust function for C ---
// Without #[no_mangle], Rust name-mangles the symbol (like _ZN...).
// C code would not be able to find it.
#[no_mangle]
pub extern "C" fn rust_hello(name: *const u8) -> i32 {
    // SAFETY: caller must provide a valid null-terminated string
    let c_str = unsafe { std::ffi::CStr::from_ptr(name as *const i8) };
    if let Ok(s) = c_str.to_str() {
        println!("Hello from Rust, {}!", s);
        s.len() as i32
    } else {
        -1
    }
}

/// A Rust function that doubles an integer, callable from C
#[no_mangle]
pub extern "C" fn rust_double(x: i32) -> i32 {
    x * 2
}

/// A Rust function that adds two floats (C ABI)
#[no_mangle]
pub extern "C" fn rust_add_floats(a: f64, b: f64) -> f64 {
    a + b
}

// --- Simulating calling C-like functions ---
// In real code you'd link a C library. Here we define matching functions
// in Rust just to demonstrate the extern "C" declaration pattern.

mod c_lib {
    // Normally this would be:
    // extern "C" {
    //     fn strlen(s: *const u8) -> usize;
    //     fn atoi(s: *const u8) -> i32;
    // }

    // We're simulating by defining Rust functions with C ABI
    #[no_mangle]
    pub extern "C" fn c_strlen(s: *const u8) -> usize {
        if s.is_null() {
            return 0;
        }
        let mut len = 0;
        unsafe {
            while *s.add(len) != 0 {
                len += 1;
            }
        }
        len
    }
}

fn main() {
    println!("=== extern \"C\" ABI ===");

    // Call our exported functions
    let result = rust_double(21);
    println!("rust_double(21) = {}", result);

    let sum = rust_add_floats(1.5, 2.7);
    println!("rust_add_floats(1.5, 2.7) = {}", sum);

    // Call the simulated C function
    let msg = b"hello\0";
    let len = unsafe { c_lib::c_strlen(msg.as_ptr()) };
    println!("c_strlen(\"hello\") = {}", len);

    // Demonstrate that we can also define function pointers with C ABI
    type CmpFunc = extern "C" fn(i32, i32) -> i32;

    extern "C" fn max_int(a: i32, b: i32) -> i32 {
        if a > b { a } else { b }
    }

    let func: CmpFunc = max_int;
    println!("max_int via C function pointer: {}", func(10, 20));

    println!();
    println!("Key points:");
    println!("  - `extern \"C\"` uses the C calling convention");
    println!("  - `#[no_mangle]` preserves symbol names for linking");
    println!("  - C string handling requires CStr for conversion");
    println!("  - FFI is inherently unsafe — the caller must uphold contracts");
    println!("  - Always check for null pointers and valid C strings");
    println!("  - For complex types, use #[repr(C)] to control layout");
}
