// 02_raw_pointers.rs — *const T, *mut T, dereferencing, and pointer arithmetic
//
// What do you do when you need to break the rules?
//
// Raw pointers (*const T and *mut T) are the Rust equivalent of C pointers.
// They can be:
//   - Null
//   - Dangling
//   - Aliased freely (no borrow checker restrictions)
//   - Created from references, allocated memory, or arbitrary addresses
//
// Rules for safe use (upheld by YOU, not the compiler):
//   - Only dereference if the pointer is valid, aligned, and non-null
//   - Follow aliasing rules (no simultaneous &mut aliasing)
//   - Ensure proper alignment
//   - Don't read uninitialized memory

fn main() {
    println!("=== Raw Pointers ===");

    // Creating raw pointers from references
    let mut value: i32 = 42;
    let const_ptr: *const i32 = &value as *const i32;
    let mut_ptr: *mut i32 = &mut value as *mut i32;

    println!("Created *const i32 and *mut i32 from references");

    unsafe {
        println!("  *const_ptr = {}", *const_ptr);
        *mut_ptr = 99;
        println!("  After *mut_ptr = 99, value = {}", value);
    }

    // --- Pointer arithmetic ---
    println!();
    println!("--- Pointer Arithmetic ---");

    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let base: *const i32 = arr.as_ptr();

    unsafe {
        // offset(n) adds n * sizeof(T) bytes — like C array indexing
        for i in 0..5 {
            let elem = *base.offset(i as isize);
            println!("  arr[{}] via offset({}) = {}", i, i, elem);
        }

        // Pointer subtraction gives distance in elements
        let third: *const i32 = base.offset(2);
        let diff = third.offset_from(base);
        println!("  offset_from(base) = {}", diff);
    }

    // --- Null vs dangling ---
    println!();
    println!("--- Null & Dangling Pointers ---");

    let null_ptr: *const i32 = std::ptr::null();
    println!("  null_ptr.is_null() = {}", null_ptr.is_null());

    // Dangling: pointing to freed memory — do NOT dereference this!
    let dangling: *const i32;
    {
        let tmp = 7;
        dangling = &tmp as *const i32;
    } // tmp dropped — dangling is now invalid
    println!("  dangling pointer created (NOT dereferenced — would be UB)");

    // --- Creating a raw pointer from heap memory ---
    println!();
    println!("--- Box to Raw Pointer ---");

    let boxed = Box::new(256i32);
    let raw: *const i32 = Box::into_raw(boxed); // leaks the allocation
    unsafe {
        println!("  Box::into_raw -> {}", *raw);
        // Must manually reconstruct the Box to free memory
        let _reboxed = Box::from_raw(raw as *mut i32);
        println!("  Memory freed via Box::from_raw");
    }

    println!();
    println!("Reminder: raw pointers give you C-like power.");
    println!("With that power you must ensure validity, alignment, and aliasing.");
}
