// 03_refcell.rs — RefCell<T>, interior mutability, borrow rules at runtime
//
// RefCell<T> moves Rust's borrow-checking from compile-time to runtime.
// With a normal reference, the compiler rejects code that could violate
// borrowing rules. With RefCell, you get exactly the same rules — one
// mutable OR many immutable references — but enforced via .borrow() and
// .borrow_mut() panicking at runtime if you break them.
//
// This is "interior mutability" — the RefCell itself is immutable (it
// implements &self methods for mutation), but the inner value can be
// mutated. Useful when external code requires &self but you need to
// mutate internal state (e.g., caching, counters, mock objects).

use std::cell::RefCell;

fn main() {
    // Create a RefCell. Even though `cell` is immutable, we can mutate
    // the inner value through borrow_mut().
    let cell = RefCell::new(String::from("hello"));

    // Mutate through &self — no &mut self needed
    {
        let mut borrow = cell.borrow_mut();
        borrow.push_str(" world");
        // borrow goes out of scope here, releasing the mutable borrow
    }

    // Read the value — borrow() gives a Ref<T> which derefs to &T
    {
        let borrow = cell.borrow();
        assert_eq!(*borrow, "hello world");
        println!("cell value: {}", borrow);
    }

    // Multiple immutable borrows are fine (just like &T)
    {
        let b1 = cell.borrow();
        let b2 = cell.borrow();
        let b3 = cell.borrow();
        println!("Three concurrent readers: {}, {}, {}", b1, b2, b3);
    }

    // Runtime panic: trying to borrow_mut while already borrowed
    // Uncomment to see the panic:
    //
    // let _b1 = cell.borrow();
    // let _b2 = cell.borrow_mut(); // PANIC: already borrowed as immutable
    //
    // Error message: "already borrowed: BorrowMutError"

    // Runtime panic: two mutable borrows simultaneously
    // Uncomment to see the panic:
    //
    // let _m1 = cell.borrow_mut();
    // let _m2 = cell.borrow_mut(); // PANIC: already borrowed as mutable
    //
    // Error message: "already borrowed: BorrowMutError"

    // Practical use case: mock objects for testing
    let mock = MockDatabase::new();
    mock.write("record1".to_string());
    mock.write("record2".to_string());
    println!("Mock writes: {:?}", mock.get_writes());

    // Practical use case: caching
    let cache = CachedComputation::new();
    println!("First call: {}", cache.compute(5));
    println!("Second call (cached): {}", cache.compute(5));
    println!("Third call (different): {}", cache.compute(10));
}

/// Mock database: records writes but provides &self API for compatibility.
struct MockDatabase {
    writes: RefCell<Vec<String>>,
}

impl MockDatabase {
    fn new() -> Self {
        MockDatabase {
            writes: RefCell::new(Vec::new()),
        }
    }

    // Takes &self (not &mut self) but still mutates internal state!
    fn write(&self, data: String) {
        self.writes.borrow_mut().push(data);
    }

    fn get_writes(&self) -> Vec<String> {
        self.writes.borrow().clone()
    }
}

/// Simple computation cache using RefCell for interior mutability.
struct CachedComputation {
    cache: RefCell<std::collections::HashMap<u64, u64>>,
}

impl CachedComputation {
    fn new() -> Self {
        CachedComputation {
            cache: RefCell::new(std::collections::HashMap::new()),
        }
    }

    // Expensive computation (just returns n*n here, but imagine real work)
    fn compute(&self, n: u64) -> u64 {
        // Check cache first
        if let Some(&result) = self.cache.borrow().get(&n) {
            println!("  (cache hit for {})", n);
            return result;
        }

        println!("  (computing {})", n);
        let result = n * n; // pretend this is expensive

        // Store in cache — mutating through &self
        self.cache.borrow_mut().insert(n, result);
        result
    }
}
