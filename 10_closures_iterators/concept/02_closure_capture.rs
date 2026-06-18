// 02_closure_capture — By reference, by value, move closures
//
// Closures can capture variables from their enclosing scope in three ways:
// - By reference (&T): borrows immutably, can be called many times
// - By mutable reference (&mut T): borrows mutably, needed to mutate
// - By value (T): takes ownership, needed when closure outlives scope
//
// Rust infers the least-permissive capture mode automatically.

fn main() {
    // --- 1. Capture by reference (immutable borrow) ---
    let name = String::from("Alice");
    let print_name = || {
        // Closure borrows `name` immutably (&String)
        println!("Name: {name}");
    };
    print_name();
    print_name();          // Can call multiple times
    println!("Still have name: {name}"); // We still own it

    // --- 2. Capture by mutable reference ---
    let mut counter = 0;
    let mut increment = || {
        // Closure borrows `counter` mutably (&mut i32)
        counter += 1;
        println!("Counter: {counter}");
    };
    increment();
    increment();
    increment();
    // Can't use `counter` here while `increment` is alive (mutable borrow exists)
    // println!("{counter}"); // ERROR: cannot borrow `counter` as immutable
    drop(increment);        // Drop the closure to release the borrow
    println!("Counter after drop: {counter}"); // Now we can use it

    // --- 3. Capture by value (move) ---
    let data = vec![1, 2, 3];
    let take = || {
        // `move` forces the closure to take ownership of `data`
        // Without `move`, Rust would try to borrow by reference,
        // but we need ownership to move into another thread or return it.
        let _owned = data; // takes ownership
        println!("Took ownership of data");
    };
    // println!("{:?}", data); // ERROR: value used after move
    take();
    // take(); // ERROR: closure can only be called once (FnOnce)

    // --- 4. The `move` keyword — force capture by value ---
    let greeting = String::from("Hello");
    let move_closure = move || {
        // `move` forces all captures to be taken by value
        println!("{greeting}");
    };
    // println!("{greeting}"); // ERROR: value moved into closure
    move_closure();
    move_closure(); // But this closure is Fn (Copy), so can be called multiple times

    // --- 5. Practical example: vector of closures ---
    let base = 10;
    let add_base = |x: i32| x + base; // captures base by reference
    let nums = vec![1, 2, 3];
    let results: Vec<i32> = nums.into_iter().map(add_base).collect();
    println!("Base {base} added to [1,2,3] = {:?}", results);

    println!("\nAll capture modes demonstrated.");
}
