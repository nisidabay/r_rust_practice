// 01_box.rs — Box<T>, heap allocation, recursive types
//
// Question: What happens when ownership rules are too strict?
// Answer: Rust gives us smart pointers — Box<T> for heap allocation and
//         recursive types, Rc<T> for shared ownership, RefCell<T> for
//         interior mutability. Each relaxes one specific rule safely.

// A recursive type needs indirection because the compiler needs to know
// the size of each type at compile time. Without Box, List would be
// infinitely sized. Box adds a pointer layer (always usize-sized),
// breaking the recursion while keeping the heap-managed data alive.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Box is also used to:
// - Transfer large data to the heap to avoid stack overflow
// - Store trait objects (dyn Trait) since the concrete type is unknown
// - Achieve type erasure (when you only care about behavior)

fn main() {
    // Box for heap-allocating a single value — rarely needed directly
    // since Rust's default allocation is already efficient, but useful
    // for large structs you want on the heap.
    let heap_val = Box::new(42);
    // Box dereferences automatically, so we can use it like an i32:
    println!("heap_val + 1 = {}", *heap_val + 1);

    // Box for recursive types — the canonical use case. Each Cons
    // holds an i32 and a pointer to the rest of the list.
    let list = Box::new(List::Cons(
        1,
        Box::new(List::Cons(
            2,
            Box::new(List::Cons(3, Box::new(List::Nil))),
        )),
    ));
    println!("Recursive list: {:?}", list);

    // Box for trait objects — store any type that implements Display
    // without knowing the concrete type at compile time.
    let data: Vec<Box<dyn std::fmt::Display>> = vec![
        Box::new(42),
        Box::new(3.14),
        Box::new("hello"),
    ];
    for item in &data {
        println!("Display: {}", item);
    }

    // Memory: Box<T> is a single pointer (8 bytes on 64-bit).
    // The data lives on the heap; the Box (pointer) lives where it's
    // declared. When the Box goes out of scope, Drop runs and frees
    // the heap memory automatically.
    println!("Size of Box<i32>: {} bytes", std::mem::size_of::<Box<i32>>());
}
