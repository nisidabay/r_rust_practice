// ex04_custom_iterator — Implement Iterator for a custom type (Fibonacci)
//
// BONUS exercise: Implement the Iterator trait for a custom struct.
// A Fibonacci iterator yields the next Fibonacci number on each call.
// Then use iterator adapters (take, filter, map) on your custom iterator.
//
// Run with: ./ex04_custom_iterator [--count N] [--filter-even]

use std::env;

/// A Fibonacci sequence iterator. Each call to next() yields the next
/// Fibonacci number: 0, 1, 1, 2, 3, 5, 8, 13, 21...
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // Store current value, compute next
        let current = self.a;
        // Check for overflow — stop before wrapping
        if self.b.checked_add(self.a).is_none() {
            return None;
        }
        self.a = self.b;
        self.b = current + self.a;
        Some(current)
    }
}

/// A stepping iterator: yields values from `start` to `end` (exclusive)
/// stepping by `step`.
struct RangeStep {
    current: i64,
    end: i64,
    step: i64,
}

impl RangeStep {
    fn new(start: i64, end: i64, step: i64) -> Self {
        RangeStep { current: start, end, step }
    }
}

impl Iterator for RangeStep {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.step > 0 && self.current >= self.end)
            || (self.step < 0 && self.current <= self.end)
        {
            return None;
        }
        let val = self.current;
        self.current = self.current + self.step;
        Some(val)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut count: usize = 10;
    let mut filter_even = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--count" => {
                i += 1;
                if i < args.len() {
                    count = args[i].parse().unwrap_or(10);
                }
            }
            "--filter-even" => {
                filter_even = true;
            }
            _ => {}
        }
        i += 1;
    }

    // --- Fibonacci iterator ---
    println!("Fibonacci sequence (first {count}):");
    let fib = Fibonacci::new();

    let fib_iter: Box<dyn Iterator<Item = u64>> = if filter_even {
        Box::new(fib.filter(|x| x % 2 == 0))
    } else {
        Box::new(fib)
    };

    let fib_nums: Vec<u64> = fib_iter.take(count).collect();
    println!("{:?}", fib_nums);

    // Verify
    if !filter_even && count >= 10 {
        assert_eq!(fib_nums[0], 0);
        assert_eq!(fib_nums[1], 1);
        assert_eq!(fib_nums[2], 1);
        assert_eq!(fib_nums[3], 2);
        assert_eq!(fib_nums[4], 3);
        assert_eq!(fib_nums[5], 5);
        assert_eq!(fib_nums[6], 8);
        assert_eq!(fib_nums[7], 13);
        assert_eq!(fib_nums[8], 21);
        assert_eq!(fib_nums[9], 34);
        println!("Fibonacci verification passed!");
    }

    // --- RangeStep iterator ---
    println!("\nRangeStep(0, 20, 3):");
    let step: Vec<i64> = RangeStep::new(0, 20, 3).collect();
    println!("{:?}", step);
    assert_eq!(step, vec![0, 3, 6, 9, 12, 15, 18]);

    println!("\nRangeStep(10, -5, -3):");
    let step_rev: Vec<i64> = RangeStep::new(10, -5, -3).collect();
    println!("{:?}", step_rev);
    assert_eq!(step_rev, vec![10, 7, 4, 1, -2]);

    println!("\nAll custom iterator tests passed!");
}
