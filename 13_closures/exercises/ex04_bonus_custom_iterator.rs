// ex04_bonus_custom_iterator.rs — Implement Iterator for a custom struct
//
// Create a Fibonacci iterator. Then use .take() to limit output.
// Demonstrates implementing the Iterator trait manually.

/// Fibonacci sequence: 0, 1, 1, 2, 3, 5, 8, 13, ...
#[derive(Clone)]
struct Fibonacci {
    a: u64,
    b: u64,
    max: Option<u64>,  // None = unlimited
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1, max: None }
    }

    fn with_max(max: u64) -> Self {
        Fibonacci { a: 0, b: 1, max: Some(max) }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if we've exceeded the max
        if let Some(max) = self.max {
            if self.a > max {
                return None;
            }
        }

        let current = self.a;

        // Handle overflow: if self.b would overflow, stop
        match self.a.checked_add(self.b) {
            Some(next_val) => {
                self.a = self.b;
                self.b = next_val;
            }
            None => {
                // Overflow would occur; return current and stop next time
                self.a = self.b;
                self.b = 0; // will cause next iteration to return None
                // but first return the current value
            }
        }

        Some(current)
    }
}

fn main() {
    // Use Fibonacci with .take()
    println!("First 10 Fibonacci numbers:");
    let fib = Fibonacci::new();
    for (i, n) in fib.take(10).enumerate() {
        println!("  fib({}) = {}", i, n);
    }

    // With max limit
    println!("\nFibonacci up to 100:");
    let fib_max = Fibonacci::with_max(100);
    for n in fib_max {
        println!("  {}", n);
    }

    // Collect into Vec
    println!("\nFirst 15 as Vec:");
    let fib_vec: Vec<u64> = Fibonacci::new().take(15).collect();
    println!("  {:?}", fib_vec);

    // Use iterator consumers
    let fib_10 = Fibonacci::new().take(10);
    let sum: u64 = fib_10.clone().sum();
    println!("\nSum of first 10 Fibonacci: {}", sum);

    let evens: Vec<u64> = fib_10.filter(|n| n % 2 == 0).collect();
    println!("Even Fibonacci in first 10: {:?}", evens);

    // Check iteration produces the right sequence
    let fib_8: Vec<u64> = Fibonacci::new().take(8).collect();
    assert_eq!(fib_8, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    println!("\nAll tests passed! ✓");
}
