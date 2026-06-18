// 04_union_ffi.rs — #[repr(C)] unions, extern "C" blocks, and FFI basics
//
// What do you do when you need to break the rules?
//
// Rust `union` fields are all stored at the same memory offset — only one
// field is "active" at a time. Accessing a union field is unsafe because
// the compiler can't verify which variant holds valid data.
//
// Unions with #[repr(C)] are useful for C-compatible FFI where a struct
// member could be one of several types (like a C union).

use std::mem;

// A C-compatible union. All fields share the same starting address.
#[repr(C)]
union IntOrFloat {
    i: i32,
    f: f32,
}

// A tagged union — tracks which field is active, like C struct + union pattern.
#[repr(C)]
struct TaggedValue {
    tag: u8, // 0 = int, 1 = float
    value: IntOrFloat,
}

impl TaggedValue {
    fn from_int(val: i32) -> Self {
        TaggedValue {
            tag: 0,
            value: IntOrFloat { i: val },
        }
    }

    fn from_float(val: f32) -> Self {
        TaggedValue {
            tag: 1,
            value: IntOrFloat { f: val },
        }
    }

    // SAFE accessor — checks the tag before reading the union field
    fn as_int(&self) -> Option<i32> {
        if self.tag == 0 {
            unsafe { Some(self.value.i) }
        } else {
            None
        }
    }

    fn as_float(&self) -> Option<f32> {
        if self.tag == 1 {
            unsafe { Some(self.value.f) }
        } else {
            None
        }
    }
}

// --- Simulated FFI extern block ---
// In real code this would reference a C library. Here we define our own
// functions below to demonstrate the pattern without linking a C compiler.
#[allow(dead_code)]
mod ffi {
    // Simulating what an extern "C" block looks like:
    extern "C" {
        // Normally: fn strlen(s: *const u8) -> usize;
        // We can't actually compile with unresolved symbols, so skip bodies.
    }
}

// Instead, let's define our own C-compatible function
extern "C" fn double_it(x: i32) -> i32 {
    x * 2
}

fn main() {
    println!("=== #[repr(C)] Unions & FFI ===");

    let tv = TaggedValue::from_int(42);
    println!("Tagged as int: {:?}", tv.as_int());
    println!("Tagged as float: {:?}", tv.as_float());

    let tv2 = TaggedValue::from_float(3.14);
    println!("Tagged as int: {:?}", tv2.as_int());
    println!("Tagged as float: {:?}", tv2.as_float());

    // Demonstrating that a union's fields overlap
    let u = IntOrFloat {
        i: 0x4048F5C3,
    }; // bit pattern for ~3.14f32
    unsafe {
        println!(
            "Union bytes reinterpreted: i = 0x{:X}, f = {}",
            u.i, u.f
        );
    }

    println!();
    println!(
        "Calling an extern \"C\" fn (defined in Rust): {}",
        double_it(21)
    );

    println!();
    println!(
        "Size of union: {} bytes (max of fields)",
        mem::size_of::<IntOrFloat>()
    );
    println!(
        "Size of tagged wrapper: {} bytes",
        mem::size_of::<TaggedValue>()
    );

    println!();
    println!("Key takeaways:");
    println!("  - #[repr(C)] unions match C union layout");
    println!("  - Accessing union fields is always unsafe");
    println!("  - The safe pattern: wrap in a tagged enum/struct");
    println!("  - extern \"C\" blocks declare functions from shared libraries");
}
