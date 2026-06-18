// 07_send_sync.rs — The Send and Sync traits, what they mean, what implements them
//
// The one question: "How do you run code in parallel — safely?"
// Answer: Rust uses Send and Sync traits as the foundation for safe concurrency.
//
// Send: A type is Send if ownership can be transferred between threads safely.
//       Most types are Send. Exceptions: Rc<T>, *const T, *mut T (raw pointers).
// Sync: A type is Sync if &T can be shared between threads safely.
//       Most types are Sync. Exceptions: Rc<T>, Cell<T>, RefCell<T>, *const T, *mut T.
//
// T: Send  =>  Arc<T> is Send/Sync
// T: Sync  =>  Mutex<T> is Send/Sync
// T: Send  =>  Mutex<T> is Sync (it's the lock that provides thread safety)

use std::cell::RefCell;
use std::marker::{Send, Sync};
use std::rc::Rc;

// These types implement Send and/or Sync:
//   - Primitive types (i32, bool, f64, etc.) → Send + Sync
//   - Box<T>, Vec<T>, String, etc. → Send + Sync if T: Send/Sync
//   - Arc<T> → Send + Sync if T: Send + Sync
//   - Mutex<T> → Send + Sync if T: Send
//   - mpsc::Sender<T> → Send
//   - mpsc::Receiver<T> → Send

// Types that are NOT Send:
//   - Rc<T> (not Send — uses non-atomic reference counting)
//   - *const T, *mut T (raw pointers — no safety guarantees)

// Types that are NOT Sync:
//   - Rc<T> (not Sync)
//   - Cell<T>, RefCell<T> (not Sync — interior mutability without thread safety)
//   - *const T, *mut T

fn main() {
    // --- Checking Send/Sync via compilation ---
    // If something doesn't compile in a thread context, it's probably not Send or Sync.

    // Rc is NOT Send — can't be moved to another thread
    let _rc = Rc::new(42);
    // This won't compile:
    // thread::spawn(move || { println!("{}", _rc); });
    // ^ error: `Rc<i32>` cannot be sent between threads safely

    // Arc IS Send — can be moved to another thread
    let _arc = std::sync::Arc::new(42);
    let handle = std::thread::spawn(move || {
        println!("Arc value in thread: {}", *_arc);
    });
    handle.join().unwrap();

    // --- RefCell is NOT Sync ---
    let cell = RefCell::new(10);
    // &RefCell<i32> can't be shared across threads
    // This won't compile:
    // thread::spawn(|| { println!("{}", cell.borrow()); });
    // ^ error: `RefCell<i32>` cannot be shared between threads safely

    // --- What makes a type Send or Sync? ---
    // You usually don't implement these manually — the compiler derives them
    // automatically if all fields are Send/Sync. But you can implement them
    // for your own types (unsafe) when you know it's safe.

    // --- Example: custom type that is Send + Sync ---
    #[derive(Debug)]
    struct ThreadSafeCounter {
        value: std::sync::atomic::AtomicU64,
    }
    // AtomicU64 is Send + Sync, so ThreadSafeCounter is automatically Send + Sync
    // (No manual impl needed)

    let counter = std::sync::Arc::new(ThreadSafeCounter {
        value: std::sync::atomic::AtomicU64::new(0),
    });

    let mut handles = vec![];
    for _ in 0..5 {
        let c = std::sync::Arc::clone(&counter);
        handles.push(std::thread::spawn(move || {
            c.value.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!(
        "Atomic counter: {}",
        counter.value.load(std::sync::atomic::Ordering::Relaxed)
    );

    // --- Marker trait demonstration ---
    // These functions are generic over Send/Sync to show which types satisfy them

    fn assert_send<T: Send>(_: &T) {}
    fn assert_sync<T: Sync>(_: &T) {}

    let x: i32 = 42;
    let s = String::from("hello");
    let v: Vec<u8> = vec![1, 2, 3];
    let a = std::sync::Arc::new(0u32);

    assert_send(&x);
    assert_sync(&x);
    assert_send(&s);
    assert_sync(&s);
    assert_send(&v);
    assert_sync(&v);
    assert_send(&a);
    assert_sync(&a);

    // Rc fails both:
    // let r = Rc::new(0u32);
    // assert_send(&r); // Would not compile — Rc is not Send
    // assert_sync(&r); // Would not compile — Rc is not Sync

    println!("All Send/Sync examples done!");
}
