// 02_reference_counting.rs — Rc<T>, clone for shared ownership
//
// Rc<T> (Reference Counted) enables multiple ownership — several parts of
// your code can hold a handle to the same heap data. An internal reference
// count tracks how many Rc handles exist; when it drops to zero, the data
// is freed. Rc is single-threaded only (no Send/Sync).
//
// Rc is like having multiple borrowers at compile time, but unlike plain
// references, Rc doesn't require a borrowed lifetime — each handle owns
// the data conceptually.

use std::rc::Rc;

fn main() {
    // Create an Rc. The data (String) is allocated on the heap.
    let original = Rc::new(String::from("shared data"));
    println!(
        "Reference count after creating original: {}",
        Rc::strong_count(&original)
    );

    // clone() on Rc does NOT copy the data — it increments the ref count
    // and returns a new handle pointing to the same heap allocation.
    let clone1 = Rc::clone(&original); // idiom: Rc::clone
    println!(
        "Reference count after clone1: {}",
        Rc::strong_count(&original)
    );

    {
        let clone2 = Rc::clone(&original);
        println!(
            "Reference count inside inner scope: {}",
            Rc::strong_count(&original)
        );
        // All handles deref to the same String:
        println!("clone2: {}", clone2);
    } // clone2 dropped here, count decreases

    println!(
        "Reference count after clone2 drops: {}",
        Rc::strong_count(&original)
    );

    // Immutable access only — Rc gives shared ownership but NOT mutable
    // access. For that, we need RefCell or Cell alongside Rc.
    println!("original: {}", original);

    // Demonstrating Rc with shared data across multiple owners
    let shared = Rc::new(String::from("shared value"));
    let handle_a = Rc::clone(&shared);
    let handle_b = Rc::clone(&shared);
    println!(
        "'{}' has {} owners",
        shared,
        Rc::strong_count(&shared)
    );
    drop(handle_a);
    println!(
        "After dropping one handle, count: {}",
        Rc::strong_count(&shared)
    );
    drop(handle_b);
    println!(
        "After dropping both extra handles, count: {}",
        Rc::strong_count(&shared)
    );
    // shared still alive — the original handle exists

    // Rc::try_unwrap consumes the Rc, returns Ok(data) if count is 1,
    // Err(Rc) if other handles still exist.
    // shared has count 1 now (only 'shared' itself remains), so this succeeds.
    let value = Rc::try_unwrap(shared).expect("should be the only owner");
    assert_eq!(value, "shared value");
    // Now the String is freed.

    // Weak pointers (Rc::downgrade) for preventing reference cycles:
    // Weak<T> holds a non-owning reference. upgrade() returns Option<Rc<T>>.
    // Used in tree structures (parent -> children via Rc, children -> parent via Weak).
    let parent = Rc::new(String::from("parent"));
    let weak_child = Rc::downgrade(&parent);
    assert_eq!(Rc::strong_count(&parent), 1);
    assert_eq!(Rc::weak_count(&parent), 1);

    // upgrade might fail if the strong reference was dropped
    if let Some(strong) = weak_child.upgrade() {
        println!("Weak upgrade succeeded: {}", strong);
    }

    drop(parent);
    assert!(weak_child.upgrade().is_none()); // parent dropped, weak is dead
}
