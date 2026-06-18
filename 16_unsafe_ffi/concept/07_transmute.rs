// 07_transmute.rs — std::mem::transmute, when to use, HIGHLY DANGEROUS
//
// What do you do when you need to break the rules?
//
// `std::mem::transmute` reinterprets the bits of a value as a different type.
// The source and destination types must have the same size.
//
// DANGER: transmute is the most dangerous tool in Rust's toolbox.
// The compiler will happily turn your data into garbage if you get the
// types wrong. Prefer safe alternatives:
//   - `as` cast for numeric conversions
//   - `bytemuck` crate for reinterpreting
//   - `bitcast` / `from_bits` for floats <-> ints

use std::mem;

fn main() {
    println!("=== std::mem::transmute ===");

    // --- Transmute between same-size types ---
    // u32 and i32 are both 4 bytes
    let unsigned: u32 = 0xFFFFFFFF;
    let signed: i32 = unsafe { mem::transmute::<u32, i32>(unsigned) };
    println!("Transmute u32 -> i32: {} -> {}", unsigned, signed);

    // Float <-> integer bit patterns (same size: 4 bytes)
    let pi: f32 = 3.14159265;
    let bits: u32 = unsafe { mem::transmute::<f32, u32>(pi) };
    let back: f32 = unsafe { mem::transmute::<u32, f32>(bits) };
    println!("Transmute f32 -> u32 -> f32: {} -> 0x{:X} -> {}", pi, bits, back);

    // --- Safer alternatives (prefer these) ---
    // f32::to_bits() and f32::from_bits() do the SAME thing safely
    let bits_safe = pi.to_bits();
    let back_safe = f32::from_bits(bits_safe);
    println!("SAFE f32::to_bits/from_bits: {} -> 0x{:X} -> {}", pi, bits_safe, back_safe);

    // --- Transmute between reference types ---
    // Very dangerous — must ensure lifetimes and aliasing rules are respected
    let val: i32 = 100;
    let ref_orig: &i32 = &val;
    // Transmuting &i32 -> &u32 is valid (same size, same alignment)
    let ref_trans: &u32 = unsafe { mem::transmute::<&i32, &u32>(ref_orig) };
    println!("Transmute &i32 -> &u32: {}", ref_trans);

    // --- Transmute a byte slice to a struct ---
    // This is how zero-copy deserialization works
    #[repr(C)]
    #[derive(Debug)]
    struct Header {
        magic: u32,   // 4 bytes
        version: u16, // 2 bytes
        flags: u16,   // 2 bytes
    } // total: 8 bytes

    let bytes: [u8; 8] = [0xDE, 0xAD, 0xBE, 0xEF, 0x01, 0x00, 0x00, 0x00];
    let header: &Header = unsafe { mem::transmute::<&[u8; 8], &Header>(&bytes) };
    println!("Transmuted bytes to Header struct: {:?}", header);

    println!();
    println!("--- Things transmute should NOT be used for ---");
    println!("  - Changing pointer types (&T -> &mut T) — UB unless uniquely owned");
    println!("  - Creating invalid enum variants");
    println!("  - Transmuting between differently-sized types (compile error)");
    println!("  - Extracting lifetimes from references (goes against Rust's guarantees)");

    println!();
    println!("Rule of thumb: if there's a safe alternative, use it.");
    println!("Transmute is for: bitcast-style reinterpretation, unsafe trait impls,");
    println!("and zero-copy deserialization where you control the data layout.");
}
