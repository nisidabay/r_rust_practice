// 03_unsafe_fns.rs — Calling and defining unsafe functions
//
// What do you do when you need to break the rules?
//
// An `unsafe fn` pushes the burden of proof to the caller:
// anyone calling it must wrap the call in `unsafe { }`.
// By contrast, a safe fn can contain an `unsafe { }` block internally,
// hiding the unsafety behind a safe API boundary.

// --- Defining an unsafe function ---
/// Returns a pointer to a static buffer (conceptually).
/// The caller must ensure the pointer is not used after the buffer is gone.
unsafe fn get_scratch_buffer() -> *mut u8 {
    // This leaks memory intentionally — the caller "owns" the buffer now.
    let buf = Box::into_raw(vec![0u8; 64].into_boxed_slice()) as *mut u8;
    buf
}

/// Unsafe function that writes to a raw pointer.
/// # Safety
/// `ptr` must be non-null, aligned, and point to valid writable memory.
unsafe fn write_byte(ptr: *mut u8, val: u8) {
    *ptr = val;
}

// --- Safe wrapper over unsafe internals ---
/// A safe API that hides unsafety from callers.
mod safe_wrapper {
    pub struct ScratchPad {
        ptr: *mut u8,
        len: usize,
    }

    impl ScratchPad {
        pub fn new() -> Self {
            // Internally we use unsafe, but keep it behind a safe boundary.
            let len = 64;
            let mut v = vec![0u8; len];
            let ptr = v.as_mut_ptr();
            // Forget the vec so it doesn't free — we'll free in Drop
            std::mem::forget(v);
            ScratchPad { ptr, len }
        }

        pub fn write(&mut self, index: usize, val: u8) {
            if index < self.len {
                unsafe {
                    *self.ptr.add(index) = val;
                }
            }
        }

        pub fn read(&self, index: usize) -> u8 {
            if index < self.len {
                unsafe { *self.ptr.add(index) }
            } else {
                0
            }
        }
    }

    impl Drop for ScratchPad {
        fn drop(&mut self) {
            unsafe {
                // Recreate the Vec to free the memory
                let _ = Vec::from_raw_parts(self.ptr, self.len, self.len);
            }
        }
    }
}

fn main() {
    println!("=== Unsafe Functions ===");

    // Calling an unsafe function requires `unsafe { }`
    println!("Calling unsafe function:");
    unsafe {
        let buf = get_scratch_buffer();
        write_byte(buf, 42);
        write_byte(buf.add(1), 100);
        println!("  Wrote bytes via unsafe fn: first byte = {}", *buf);
        // Note: we should free this — omitting for brevity in concept
    }

    println!();
    println!("Using safe wrapper (unsafe hidden inside):");
    let mut pad = safe_wrapper::ScratchPad::new();
    pad.write(0, 7);
    pad.write(1, 14);
    println!("  pad[0] = {}, pad[1] = {}", pad.read(0), pad.read(1));

    println!();
    println!("Takeaway: prefer safe wrappers over exposing unsafe APIs.");
    println!("`unsafe fn` is necessary for FFI declarations and low-level primitives.");
}
