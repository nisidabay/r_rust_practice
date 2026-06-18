// ex02: Convert between CString and *const i8 safely
//
// When calling C functions from Rust, you often need to pass strings.
// CString (from std::ffi) owns a null-terminated C-compatible buffer.
//
// Your task: implement safe wrappers for CString conversion.

use std::ffi::{CStr, CString};
use std::ptr;

/// Convert a Rust &str to a CString and return a raw pointer to the
/// C-compatible buffer. The caller must ensure the CString outlives
/// the returned pointer.
fn to_c_ptr(s: &str) -> (*const i8, CString) {
    let c_str = CString::new(s).expect("CString::new failed (embedded null?)");
    let ptr = c_str.as_ptr();
    (ptr, c_str) // return CString so it's not dropped
}

/// Safely convert a C string pointer back to a Rust &str.
/// Returns None if the pointer is null or the bytes aren't valid UTF-8.
fn from_c_ptr(ptr: *const i8) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let c_str = unsafe { CStr::from_ptr(ptr) };
    c_str.to_str().ok().map(|s| s.to_string())
}

/// Count the length of a C string (excluding null terminator).
/// Returns 0 if the pointer is null.
fn c_str_len(ptr: *const i8) -> usize {
    if ptr.is_null() {
        return 0;
    }
    unsafe {
        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        len
    }
}

fn main() {
    println!("=== Exercise 02: CString / CStr conversion ===");

    // --- Rust -> C string -> Rust ---
    let original = "Hello, FFI!";
    let (c_ptr, _cstring_owner) = to_c_ptr(original);

    println!("Original:  \"{}\"", original);
    println!("C pointer: {:p}", c_ptr);

    let roundtripped = from_c_ptr(c_ptr).unwrap();
    println!("Roundtrip: \"{}\"", roundtripped);
    assert_eq!(original, roundtripped);

    // Check length
    let len = c_str_len(c_ptr);
    println!("C string length: {} (expected {})", len, original.len());
    assert_eq!(len, original.len());

    // --- Null pointer safety ---
    let null_result = from_c_ptr(ptr::null());
    println!("Null pointer -> {:?}", null_result);
    assert!(null_result.is_none());

    let zero_len = c_str_len(ptr::null());
    assert_eq!(zero_len, 0);

    // --- Embedded null (error case) ---
    let bad = "hello\0world";
    let cstring_result = CString::new(bad);
    println!("CString::new with embedded null: {:?}", cstring_result);
    assert!(cstring_result.is_err());

    println!();
    println!("All assertions passed! ✓");

    println!();
    println!("Key points:");
    println!("  - CString owns a null-terminated buffer; keep it alive while ptr is used");
    println!("  - CStr is a borrowed view into a C string (like &str for C strings)");
    println!("  - CString::new fails if the string contains internal null bytes");
    println!("  - Always check for null pointers from C before dereferencing");
}
